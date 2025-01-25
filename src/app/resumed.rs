use std::rc::Rc;

use super::App;
use crate::canvas::Canvas;
use winit::event_loop::ActiveEventLoop;

impl App {
    // This is a common indicator that you can create a window.
    pub fn resumed_func(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(self.attributes.clone()).unwrap();
        let win_rc = Rc::new(window);
        let context = softbuffer::Context::new(win_rc.clone()).unwrap();
        let surface = softbuffer::Surface::new(&context, win_rc.clone()).unwrap();
        self.window = Some(win_rc);
        let window = self.window.as_ref().unwrap();

        let monitor = window
            .current_monitor()
            .unwrap_or_else(|| window.primary_monitor().unwrap());

        self.assign_monitor(&monitor);

        let window_phisical = self.window.as_ref().unwrap().inner_size();
        self.canvas = Some(Canvas::new(
            surface,
            (window_phisical.width as i32, window_phisical.height as i32),
        ));

        self.update_circle_cursor(event_loop);
    }
}
