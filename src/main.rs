#![windows_subsystem = "windows"]
use sketchover::app::{App, UserEvent};
use winit::{error::EventLoopError, event_loop::EventLoop};

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();
    let proxy = event_loop.create_proxy();

    let mut app = App::new(proxy);

    event_loop.run_app(&mut app)
}
