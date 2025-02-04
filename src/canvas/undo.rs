use super::{Canvas, Preview};

impl Canvas {
    pub fn redo(&mut self) {
        if let Some(buffer) = self.redo_history.pop() {
            self.undo_history.push(self.compress_current_state());
            self.decompress_state(buffer);
            self.rerender();
        }
    }

    pub fn undo(&mut self) {
        if let Some(buffer) = self.undo_history.pop() {
            self.redo_history.push(self.compress_current_state());
            self.decompress_state(buffer);
            self.rerender();
        }

        if self.is_clicked && self.preview == Preview::None {
            // Add to hist because we don't want to lose the redo history
            self.add_to_hist();
        }
    }

    fn compress_current_state(&self) -> Vec<(u32, u32)> {
        let mut compressed = Vec::new();
        let mut actual_color = self.drawing[0];
        let mut color_count = 0;

        for &color in &self.drawing {
            if color == actual_color {
                color_count += 1;
            } else {
                compressed.push((color_count, actual_color));
                actual_color = color;
                color_count = 1;
            }
        }

        compressed.push((color_count, actual_color));
        compressed
    }

    fn decompress_state(&mut self, compressed: Vec<(u32, u32)>) {
        let mut index = 0;
        for (count, color) in compressed {
            self.drawing[index..index + count as usize].fill(color);
            index += count as usize;
        }
    }

    pub fn save_state(&mut self) {
        self.redo_history.clear();
        self.add_to_hist();
    }

    // Does not delete the redo history
    fn add_to_hist(&mut self) {
        if self.undo_history.len() >= self.max_history {
            self.undo_history.remove(0);
        }

        self.undo_history.push(self.compress_current_state());
    }
}
