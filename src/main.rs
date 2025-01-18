use sketchover::app::{App, UserEvent};
use tray_icon::{
    menu::{IsMenuItem, Menu, MenuItemBuilder, PredefinedMenuItem},
    TrayIconBuilder,
};
use winit::{
    error::EventLoopError, event_loop::EventLoop, platform::windows::WindowAttributesExtWindows,
    window::WindowAttributes,
};

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();

    let menu_items: [&dyn IsMenuItem; 4] = [
        &MenuItemBuilder::new()
            .text("Show")
            .id(tray_icon::menu::MenuId::from("Show"))
            .enabled(true)
            .build(),
        &MenuItemBuilder::new()
            .text("Hide")
            .id(tray_icon::menu::MenuId::from("Hide"))
            .enabled(true)
            .build(),
        &PredefinedMenuItem::separator(),
        &MenuItemBuilder::new()
            .text("Exit")
            .id(tray_icon::menu::MenuId::from("Exit"))
            .enabled(true)
            .build(),
    ];

    let tray_menu = Menu::with_items(&menu_items).unwrap();

    let proxy = event_loop.create_proxy();
    let proxy_clone = proxy.clone();
    tray_icon::TrayIconEvent::set_event_handler(Some(move |event| {
        proxy.send_event(UserEvent::TrayIconEvent(event)).unwrap();
    }));

    tray_icon::menu::MenuEvent::set_event_handler(Some(move |event| {
        proxy_clone.send_event(UserEvent::MenuEvent(event)).unwrap();
    }));

    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("SketchOver")
        .with_icon(tray_icon::Icon::from_path("assets/icon.ico", Some((32, 32))).unwrap())
        .build()
        .unwrap();

    let attributes = WindowAttributes::default()
        .with_title("My Window")
        .with_transparent(true)
        .with_decorations(false)
        .with_maximized(true)
        .with_resizable(false)
        .with_skip_taskbar(true)
        .with_visible(false)
        .with_window_level(winit::window::WindowLevel::AlwaysOnTop);

    let mut app = App::new(attributes);

    event_loop.run_app(&mut app)
}
