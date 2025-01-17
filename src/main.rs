use sketchover::app::App;
use winit::{error::EventLoopError, event_loop::EventLoop, window::WindowAttributes};

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new().unwrap();

    let attributes = WindowAttributes::default()
        .with_title("My Window")
        .with_transparent(true)
        .with_decorations(false)
        .with_maximized(true)
        .with_resizable(false)
        .with_window_level(winit::window::WindowLevel::AlwaysOnTop);

    let mut app = App::new(attributes);

    event_loop.run_app(&mut app)
}
