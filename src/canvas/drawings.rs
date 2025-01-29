use super::Canvas;
use crate::app::Mode;

impl Canvas {
    pub fn resize_radius(&mut self, delta: f64) {
        self.radius = (self.radius + delta).max(1.0).min(20.0);
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn get_mode(&self) -> Mode {
        self.mode.clone()
    }

    pub fn toggle_mode(&mut self) {
        match self.mode {
            Mode::Drawing(color) => self.mode = Mode::Erasing(color),
            Mode::Erasing(color) => self.mode = Mode::Drawing(color),
        }
    }

    pub fn paint_line(&mut self, x1: i32, y1: i32) {
        let (x0, y0) = self.cursor_pos;
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();

        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut err = dx - dy;
        let mut x = x0;
        let mut y = y0;

        while x != x1 || y != y1 {
            self.paint_circle(x, y);

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
        self.paint_circle(x, y);
    }

    pub fn paint_circle(&mut self, center_x: i32, center_y: i32) {
        let radius = self.radius as i32;
        let mut x = radius;
        let mut y = 0;
        let mut decision = 1 - radius;

        while x >= y {
            self.draw_horizontal_line(center_x - x, center_x + x, center_y + y);
            self.draw_horizontal_line(center_x - x, center_x + x, center_y - y);
            self.draw_horizontal_line(center_x - y, center_x + y, center_y + x);
            self.draw_horizontal_line(center_x - y, center_x + y, center_y - x);

            y += 1;

            if decision <= 0 {
                decision += 2 * y + 1;
            } else {
                x -= 1;
                decision += 2 * (y - x) + 1;
            }
        }
    }

    #[inline]
    fn draw_horizontal_line(&mut self, x1: i32, x2: i32, y: i32) {
        if let Ok(mut buffer) = self.surface.buffer_mut() {
            let (width, height) = self.window_size;
            if y < 0 || y >= height {
                return;
            }

            let start_x = (y * self.window_size.0 + x1.max(0).min(width - 1)) as usize;
            let end_x = (y * self.window_size.0 + x2.max(0).min(width - 1)) as usize;

            let (drawing_color, buffer_color) = if let Mode::Drawing(color) = self.mode {
                (color as u32, color as u32)
            } else {
                (0, self.background_color as u32)
            };

            self.drawing[start_x..end_x].fill(drawing_color);
            buffer[start_x..end_x].fill(buffer_color);
        }
    }
}
