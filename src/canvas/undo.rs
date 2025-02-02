use super::Canvas;

impl Canvas {
    pub fn undo(&mut self) {
        if let Some(buffer) = self.history.pop() {
            let mut index = 0;
            for (count, color) in buffer {
                self.drawing[index..index + count as usize].fill(color);
                index += count as usize;
            }

            self.rerender();
        } else {
            self.clear();
        }
    }

    pub fn save_state(&mut self) {
        if self.history.len() >= self.max_history {
            self.history.remove(0);
        }

        let mut current_drawing = Vec::new();
        let mut actual_color = self.drawing[0];
        let mut color_count = 0;

        for &color in &self.drawing {
            if color == actual_color {
                color_count += 1;
            } else {
                current_drawing.push((color_count, actual_color));
                actual_color = color;
                color_count = 1;
            }
        }

        current_drawing.push((color_count, actual_color));

        self.history.push(current_drawing);
    }
}
