use sketchover::app::{App, UserEvent};
use tray_icon::TrayIconBuilder;
use winit::{
    error::EventLoopError, event_loop::EventLoop, platform::windows::WindowAttributesExtWindows,
    window::WindowAttributes,
};

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();

    let _tray_icon = TrayIconBuilder::new()
        .with_tooltip("SketchOver")
        .with_icon(tray_icon::Icon::from_path("assets/icon.ico", Some((32, 32))).unwrap())
        .build()
        .unwrap();

    let proxy = event_loop.create_proxy();
    tray_icon::TrayIconEvent::set_event_handler(Some(move |event| {
        proxy.send_event(UserEvent::TrayIconEvent(event)).unwrap();
    }));

    let proxy = event_loop.create_proxy();
    tray_icon::menu::MenuEvent::set_event_handler(Some(move |event| {
        proxy.send_event(UserEvent::MenuEvent(event)).unwrap();
    }));

    let attributes = WindowAttributes::default()
        .with_title("My Window")
        .with_transparent(true)
        .with_decorations(false)
        .with_maximized(true)
        .with_resizable(false)
        .with_skip_taskbar(true)
        .with_window_level(winit::window::WindowLevel::AlwaysOnTop);

    let mut app = App::new(attributes);

    event_loop.run_app(&mut app)
}
