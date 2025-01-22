#[derive(Default)]
pub struct Canvas {
    pub buffer: Vec<u8>,
    pub window_size: (u32, u32),
}

impl Canvas {
    pub fn new(window_size: (u32, u32)) -> Canvas {
        Canvas {
            buffer: vec![0; (window_size.0 * window_size.1 * 4) as usize],
            window_size,
        }
    }

    pub fn paint_line(&mut self, x0: f64, y0: f64, x1: f64, y1: f64, radius: f64) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();

        let sx = if x0 < x1 { 1.0 } else { -1.0 };
        let sy = if y0 < y1 { 1.0 } else { -1.0 };

        let mut err = dx - dy;
        let mut x = x0;
        let mut y = y0;

        while x != x1 || y != y1 {
            self.paint_circle(x, y, radius);

            let e2 = 2.0 * err;

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

    pub fn paint_circle(&mut self, center_x: f64, center_y: f64, radius: f64) {
        let mut x = radius;
        let mut y = 0.0;
        let mut decision = 1.0 - radius;

        while x >= y {
            // Draw horizontal lines to fill the circle
            for i in (-x as i32)..=(x as i32) {
                self.paint_pixel(center_x + i as f64, center_y + y);
                self.paint_pixel(center_x + i as f64, center_y - y);
            }
            for i in (-y as i32)..=(y as i32) {
                self.paint_pixel(center_x + i as f64, center_y + x);
                self.paint_pixel(center_x + i as f64, center_y - x);
            }

            y += 1.0;

            if decision <= 0.0 {
                decision += 2.0 * y + 1.0;
            } else {
                x -= 1.0;
                decision += 2.0 * (y - x) + 1.0;
            }
        }
    }

    fn paint_pixel(&mut self, x: f64, y: f64) {
        let (width, height) = self.window_size;

        // Bounds checking
        if x < 0.0 || y < 0.0 || x >= width as f64 || y >= height as f64 {
            return;
        }

        let index = ((y as u32 * width + x as u32) * 4) as usize;

        // Write all color components at once using array slice
        self.buffer[index..index + 4].copy_from_slice(&[255, 0, 0, 255]);
    }

    pub fn clear(&mut self) {
        self.buffer.iter_mut().for_each(|x| *x = 0);
    }

    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }
}
