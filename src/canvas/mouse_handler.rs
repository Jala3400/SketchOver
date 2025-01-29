use winit::event::{ElementState, MouseButton};

use super::Canvas;

impl Canvas {
    pub fn cursor_moved(&mut self, x: f64, y: f64) {
        let x = x as i32;
        let y = y as i32;
        if self.is_clicked {
            self.paint_line(x, y);
        }
        self.cursor_pos = (x, y);
    }

    pub fn mouse_pressed(&mut self, state: ElementState, button: MouseButton) {
        if state == ElementState::Pressed {
            match button {
                MouseButton::Left => {
                    self.is_clicked = true;
                    self.paint_circle(self.cursor_pos.0 as i32, self.cursor_pos.1 as i32);
                }
                _ => (),
            }
        } else {
            match button {
                MouseButton::Left => {
                    self.is_clicked = false;
                }
                _ => (),
            }
        }
    }
}
