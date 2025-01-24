use softbuffer::Surface;
use std::rc::Rc;
use winit::window::Window;

pub struct Canvas {
    drawing: Vec<u32>,
    surface: Surface<Rc<Window>, Rc<Window>>,
    window_size: (i32, i32),
}

impl Canvas {
    pub fn new(surface: Surface<Rc<Window>, Rc<Window>>, window_size: (i32, i32)) -> Canvas {
        Canvas {
            drawing: vec![0; (window_size.0 * window_size.1) as usize],
            surface: surface,
            window_size,
        }
    }
    pub fn paint_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, radius: i32) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();

        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut err = dx - dy;
        let mut x = x0;
        let mut y = y0;

        while x != x1 || y != y1 {
            self.paint_circle(x, y, radius);

            let e2 = 2 * err;

            if e2 > -dy {
                err -= dy;
                x += sx;
            }

            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
        self.paint_circle(x, y, radius);
    }

    pub fn paint_circle(&mut self, center_x: i32, center_y: i32, radius: i32) {
        let mut x = radius;
        let mut y = 0;
        let mut decision = 1 - radius;

        while x >= y {
            for i in -x..=x {
                self.paint_pixel(center_x + i, center_y + y);
                self.paint_pixel(center_x + i, center_y - y);
            }
            for i in -y..=y {
                self.paint_pixel(center_x + i, center_y + x);
                self.paint_pixel(center_x + i, center_y - x);
            }

            y += 1;

            if decision <= 0 {
                decision += 2 * y + 1;
            } else {
                x -= 1;
                decision += 2 * (y - x) + 1;
            }
        }
    }

    fn paint_pixel(&mut self, x: i32, y: i32) {
        let (width, height) = self.window_size;

        // Bounds checking
        if x < 0 || y < 0 || x >= width || y >= height {
            return;
        }

        let index = (y * width + x) as usize;
        // Transparency goes first
        let color = 0xffff0000;
        self.drawing[index] = color;
        let mut buffer = self.surface.buffer_mut().unwrap();
        buffer[index] = color;
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        let mut new_buffer = vec![0; (width * height) as usize];
        let (old_width, old_height) = self.window_size;

        for y in 0..height.min(old_height) {
            let old_row_start = (y * old_width) as usize;
            let new_row_start = (y * width) as usize;
            let old_pixel = self.drawing[old_row_start];
            new_buffer[new_row_start] = old_pixel;
            if let Ok(mut buffer) = self.surface.buffer_mut() {
                buffer[new_row_start] = old_pixel;
            }
        }

        let width_n = std::num::NonZeroU32::new(width as u32).unwrap();
        let height_n = std::num::NonZeroU32::new(height as u32).unwrap();
        let _ = self.surface.resize(width_n, height_n);

        self.drawing = new_buffer;
        self.window_size = (width, height);
    }

    pub fn clear(&mut self) {
        self.drawing.iter_mut().for_each(|x| *x = 0);
        let mut buffer = self.surface.buffer_mut().unwrap();
        buffer.iter_mut().for_each(|x| *x = 0);
    }

    pub fn redraw(&mut self) {
        let buffer = self.surface.buffer_mut().unwrap();

        // Copy canvas buffer into pixels frame
        // buffer.copy_from_slice(self.canvas.buffer());

        // Render the frame
        let _ = buffer.present();
    }

    pub fn buffer(&self) -> &[u32] {
        &self.drawing
    }
}
