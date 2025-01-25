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

    pub fn paint_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, radius: i32, color: u32) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();

        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut err = dx - dy;
        let mut x = x0;
        let mut y = y0;

        while x != x1 || y != y1 {
            self.paint_circle(x, y, radius, color);

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
        self.paint_circle(x, y, radius, color);
    }

    pub fn paint_circle(&mut self, center_x: i32, center_y: i32, radius: i32, color: u32) {
        let mut x = radius;
        let mut y = 0;
        let mut decision = 1 - radius;

        while x >= y {
            for i in -x..=x {
                self.paint_pixel(center_x + i, center_y + y, color);
                self.paint_pixel(center_x + i, center_y - y, color);
            }
            for i in -y..=y {
                self.paint_pixel(center_x + i, center_y + x, color);
                self.paint_pixel(center_x + i, center_y - x, color);
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

    fn paint_pixel(&mut self, x: i32, y: i32, color: u32) {
        if let Ok(mut buffer) = self.surface.buffer_mut() {
            let (width, height) = self.window_size;

            // Bounds checking
            if x < 0 || y < 0 || x >= width || y >= height {
                return;
            }

            let index = (y * width + x) as usize;
            // Transparency goes first
            self.drawing[index] = color;
            buffer[index] = color;
        }
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        let width_n = std::num::NonZeroU32::new(width as u32).unwrap();
        let height_n = std::num::NonZeroU32::new(height as u32).unwrap();

        let _ = self.surface.resize(width_n, height_n);
        if let Ok(mut buffer) = self.surface.buffer_mut() {
            let (old_width, old_height) = self.window_size;

            let mut new_buffer = vec![0; (width * height) as usize];

            let min_height = height.min(old_height);
            let min_width = width.min(old_width);

            for y in 0..min_height {
                let old_start = (y * old_width) as usize;
                let new_start = (y * width) as usize;

                new_buffer[new_start..new_start + min_width as usize]
                    .copy_from_slice(&self.drawing[old_start..old_start + min_width as usize]);
            }

            self.drawing = new_buffer.clone();
            buffer.copy_from_slice(&new_buffer);
            self.window_size = (width, height);
        }
    }

    pub fn clear(&mut self) {
        if let Ok(mut buffer) = self.surface.buffer_mut() {
            self.drawing.iter_mut().for_each(|x| *x = 0);
            buffer.iter_mut().for_each(|x| *x = 0);
        }
    }

    // fn rerender(&mut self) {
    //     if let Ok(mut buffer) = self.surface.buffer_mut() {
    //         buffer
    //             .iter_mut()
    //             .for_each(|x| *x = self.background_color as u32);
    //         self.drawing
    //             .iter()
    //             .enumerate()
    //             .for_each(|(i, x)| buffer[i] = *x);
    //     }
    // }

    pub fn redraw(&mut self) {
        if let Ok(buffer) = self.surface.buffer_mut() {
            // Copy canvas buffer into pixels frame
            // buffer.copy_from_slice(self.canvas.buffer());

            // Render the frame
            let _ = buffer.present();
        }
    }

    pub fn buffer(&self) -> &[u32] {
        &self.drawing
    }
}
