use super::Canvas;

impl Canvas {
    pub fn undo(&mut self) {
        if let Some(buffer) = self.history.pop() {
            self.drawing = buffer;
            self.rerender();
        }
    }

    pub fn save_state(&mut self) {
        if self.history.len() >= self.max_history {
            self.history.remove(0);
        }

        self.history.push(self.drawing.clone());
    }
}
