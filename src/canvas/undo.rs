use super::Canvas;

impl Canvas {
    pub fn undo(&mut self) {
        if let Some(buffer) = self.history.pop() {
            let mut index = 0;
            for (color, count) in buffer {
                for _ in 0..count {
                    self.drawing[index] = color;
                    index += 1;
                }
            }

            self.rerender();
        }
    }

    pub fn save_state(&mut self) {
        if self.history.len() >= self.max_history {
            self.history.remove(0);
        }

        let mut current_drawing = Vec::new();
        let mut actual_color = self.drawing[0];
        let mut color_count = 0;

        for color in self.drawing.iter() {
            if *color == actual_color {
                color_count += 1;
            } else {
                current_drawing.push((actual_color, color_count));
                actual_color = *color;
                color_count = 1;
            }
        }

        self.history.push(current_drawing);
    }
}
