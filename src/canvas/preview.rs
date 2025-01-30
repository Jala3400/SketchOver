use super::{Canvas, Preview};

impl Canvas {
    pub fn update_line_preview(&mut self, x1: i32, y1: i32) {
        self.delete_line_preview();
        self.paint_line_preview(x1, y1);
    }

    fn delete_line_preview(&mut self) {
        if let Preview::Line(x0, y0) = self.preview {
            let (x1, y1) = self.cursor_pos;
            let dx = (x1 - x0).abs();
            let dy = (y1 - y0).abs();

            let sx = if x0 < x1 { 1 } else { -1 };
            let sy = if y0 < y1 { 1 } else { -1 };

            let mut err = dx - dy;
            let mut x = x0;
            let mut y = y0;

            while x != x1 || y != y1 {
                self.delete_circle_preview(x, y);

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
        }
    }

    fn delete_circle_preview(&mut self, center_x: i32, center_y: i32) {
        let radius = self.radius as i32;
        let mut x = radius;
        let mut y = 0;
        let mut decision = 1 - radius;

        while x >= y {
            self.delete_horizontal_line_preview(center_x - x, center_x + x, center_y + y);
            self.delete_horizontal_line_preview(center_x - x, center_x + x, center_y - y);
            self.delete_horizontal_line_preview(center_x - y, center_x + y, center_y + x);
            self.delete_horizontal_line_preview(center_x - y, center_x + y, center_y - x);

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
    fn delete_horizontal_line_preview(&mut self, x1: i32, x2: i32, y: i32) {
        if let Ok(mut buffer) = self.surface.buffer_mut() {
            let (width, height) = self.window_size;
            if y < 0 || y >= height {
                return;
            }

            let start_x = (y * self.window_size.0 + x1.max(0).min(width - 1)) as usize;
            let end_x = (y * self.window_size.0 + x2.max(0).min(width - 1)) as usize;

            buffer[start_x..end_x]
                .iter_mut()
                .zip(self.drawing[start_x..end_x].iter())
                .for_each(|(buf, &color)| {
                    *buf = if color == 0 {
                        self.background_color as u32
                    } else {
                        color
                    }
                });
        }
    }

    fn paint_line_preview(&mut self, x1: i32, y1: i32) {
        if let Preview::Line(x0, y0) = self.preview {
            let dx = (x1 - x0).abs();
            let dy = (y1 - y0).abs();

            let sx = if x0 < x1 { 1 } else { -1 };
            let sy = if y0 < y1 { 1 } else { -1 };

            let mut err = dx - dy;
            let mut x = x0;
            let mut y = y0;

            while x != x1 || y != y1 {
                self.paint_circle_preview(x, y);
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
        }
    }

    fn paint_circle_preview(&mut self, center_x: i32, center_y: i32) {
        let radius = self.radius as i32;
        let mut x = radius;
        let mut y = 0;
        let mut decision = 1 - radius;

        while x >= y {
            self.draw_horizontal_line_preview(center_x - x, center_x + x, center_y + y);
            self.draw_horizontal_line_preview(center_x - x, center_x + x, center_y - y);
            self.draw_horizontal_line_preview(center_x - y, center_x + y, center_y + x);
            self.draw_horizontal_line_preview(center_x - y, center_x + y, center_y - x);

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
    fn draw_horizontal_line_preview(&mut self, x1: i32, x2: i32, y: i32) {
        let drawing_color = self.get_drawing_color();
        if let Ok(mut buffer) = self.surface.buffer_mut() {
            let (width, height) = self.window_size;
            if y < 0 || y >= height {
                return;
            }

            let start_x = (y * self.window_size.0 + x1.max(0).min(width - 1)) as usize;
            let end_x = (y * self.window_size.0 + x2.max(0).min(width - 1)) as usize;

            buffer[start_x..end_x].fill(drawing_color);
        }
    }
}
