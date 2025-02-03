use winit::event::{ElementState, MouseButton};

use super::{Canvas, Preview};

impl Canvas {
    pub fn cursor_moved(&mut self, x: f64, y: f64) {
        let x = x as i32;
        let y = y as i32;
        if self.is_clicked {
            match self.preview {
                Preview::Line(_, _) => self.update_line_preview(x, y),
                Preview::Square(_, _) => self.update_square_preview(x, y),
                Preview::None => self.paint_line(x, y),
            }
        }
        self.cursor_pos = (x, y);
    }

    pub fn mouse_pressed(
        &mut self,
        state: ElementState,
        button: MouseButton,
        modifiers: winit::keyboard::ModifiersState,
    ) {
        if state == ElementState::Pressed {
            match button {
                MouseButton::Left => {
                    self.is_clicked = true;
                    match modifiers {
                        winit::keyboard::ModifiersState::SHIFT => {
                            self.preview = Preview::Line(self.cursor_pos.0, self.cursor_pos.1);
                        }
                        winit::keyboard::ModifiersState::CONTROL => {
                            self.preview = Preview::Square(self.cursor_pos.0, self.cursor_pos.1);
                        }
                        _ => {
                            // Only save state if we are not previewing a shape
                            // It will be saved when the shape is drawn
                            // This is to prevent issues when pressing ctrl z while previewing
                            self.save_state();
                            self.paint_circle(self.cursor_pos.0 as i32, self.cursor_pos.1 as i32);
                        }
                    }
                }
                _ => (),
            }
        } else {
            match button {
                MouseButton::Left => {
                    match self.preview {
                        Preview::Line(x, y) => {
                            self.save_state();
                            self.paint_line(x, y);
                        }
                        Preview::Square(x, y) => {
                            self.save_state();
                            self.paint_square(x, y);
                        }
                        _ => (),
                    }
                    self.preview = Preview::None;
                    self.is_clicked = false;
                }
                _ => (),
            }
        }
    }
}
