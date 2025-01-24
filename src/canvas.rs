#[derive(Default)]
pub struct Canvas {
    pub buffer: Vec<u8>,
    pub window_size: (i32, i32),
}

impl Canvas {
    pub fn new(window_size: (i32, i32)) -> Canvas {
        Canvas {
            buffer: vec![0; (window_size.0 * window_size.1 * 4) as usize],
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
        if x < 0 || y < 0 || x >= width as i32 || y >= height as i32 {
            return;
        }

        let index = ((y * width + x) * 4) as usize;
        self.buffer[index..index + 4].copy_from_slice(&[255, 0, 0, 255]);
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        let mut new_buffer = vec![0; (width * height * 4) as usize];
        let (old_width, old_height) = self.window_size;

        for y in 0..height.min(old_height) {
            let old_row_start = (y * old_width * 4) as usize;
            let new_row_start = (y * width * 4) as usize;
            let row_width = width.min(old_width) * 4;
            new_buffer[new_row_start..new_row_start + row_width as usize]
                .copy_from_slice(&self.buffer[old_row_start..old_row_start + row_width as usize]);
        }

        self.buffer = new_buffer;
        self.window_size = (width, height);
    }

    pub fn clear(&mut self) {
        self.buffer.iter_mut().for_each(|x| *x = 0);
    }

    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }
}
