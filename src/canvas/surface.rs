use super::Canvas;
use crate::app::{Colors, Mode};

impl Canvas {
    pub fn clear(&mut self) {
        if let Ok(mut buffer) = self.surface.buffer_mut() {
            self.drawing.iter_mut().for_each(|x| *x = 0);
            buffer
                .iter_mut()
                .for_each(|x| *x = self.background_color as u32);
        }
    }

    pub fn set_background_color(&mut self, color: Colors) {
        self.background_color = color;
        self.rerender();
    }

    pub fn set_opacity(&mut self, opacity: u8) {
        if let Ok(mut buffer) = self.surface.buffer_mut() {
            let bg_color = if self.background_color as u32 == 0 {
                0
            } else {
                (self.background_color as u32 & 0x00FFFFFF) | ((opacity as u32) << 24)
            };

            buffer
                .iter_mut()
                .zip(self.drawing.iter())
                .for_each(|(buf, &color)| {
                    *buf = if color == 0 {
                        bg_color
                    } else {
                        (color & 0x00FFFFFF) | ((opacity as u32) << 24)
                    };
                });
        }
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        let width_n = std::num::NonZeroU32::new(width as u32).unwrap();
        let height_n = std::num::NonZeroU32::new(height as u32).unwrap();
        let _ = self.surface.resize(width_n, height_n);

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
        self.window_size = (width, height);
        self.rerender();
    }

    pub fn reset(&mut self) {
        if let Ok(mut buffer) = self.surface.buffer_mut() {
            self.mode = Mode::Drawing;
            self.current_color = Colors::RED;
            self.radius = 2.0;
            buffer.fill(0);
            self.drawing.fill(0);
            self.background_color = Colors::TRANSPARENT;
        }
    }

    fn rerender(&mut self) {
        if let Ok(mut buffer) = self.surface.buffer_mut() {
            buffer
                .iter_mut()
                .zip(self.drawing.iter())
                .for_each(|(buf, &color)| {
                    *buf = if color == 0 {
                        self.background_color as u32
                    } else {
                        color
                    };
                });
            self.update_preview();
        }
    }

    pub fn redraw(&mut self) {
        if let Ok(buffer) = self.surface.buffer_mut() {
            let _ = buffer.present();
        }
    }
}
