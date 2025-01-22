use sketchover::{
    app::{App, UserEvent},
    hotkeys::HotkeyManager,
    tray_icon::setup_tray_icon,
};
use winit::{
    error::EventLoopError, event_loop::EventLoop, platform::windows::WindowAttributesExtWindows,
    window::WindowAttributes,
};

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();
    let proxy = event_loop.create_proxy();

    let _tray_icon = setup_tray_icon(&proxy);

    let hotkey_manager = HotkeyManager::new(&proxy);

    let attributes = WindowAttributes::default()
        .with_title("My Window")
        .with_transparent(true)
        .with_decorations(false)
        .with_maximized(true)
        .with_resizable(false)
        .with_skip_taskbar(true)
        .with_visible(false)
        .with_window_level(winit::window::WindowLevel::AlwaysOnTop);

    let mut app = App::new(attributes, hotkey_manager);

    event_loop.run_app(&mut app)
}
