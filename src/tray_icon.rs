use tray_icon::{
    menu::{IsMenuItem, Menu, MenuItemBuilder, PredefinedMenuItem},
    Icon, TrayIconBuilder,
};
use winit::event_loop::EventLoopProxy;

use crate::app::UserEvent;

pub fn setup_tray_icon(proxy: &EventLoopProxy<UserEvent>) -> tray_icon::TrayIcon {
    let menu_items: [&dyn IsMenuItem; 7] = [
        &MenuItemBuilder::new()
            .text("Transparent to mouse")
            .id(tray_icon::menu::MenuId::from("Transparent to mouse"))
            .enabled(true)
            .accelerator(Some("Ctrl+Alt+T"))
            .unwrap()
            .build(),
        &PredefinedMenuItem::separator(),
        &MenuItemBuilder::new()
            .text("New canvas")
            .id(tray_icon::menu::MenuId::from("New canvas"))
            .enabled(true)
            .accelerator(Some("Ctrl+Alt+S"))
            .unwrap()
            .build(),
        &MenuItemBuilder::new()
            .text("Show previous")
            .id(tray_icon::menu::MenuId::from("Show previous"))
            .enabled(true)
            .accelerator(Some("Ctrl+Alt+Shift+S"))
            .unwrap()
            .build(),
        &MenuItemBuilder::new()
            .text("Hide")
            .id(tray_icon::menu::MenuId::from("Hide"))
            .enabled(true)
            .accelerator(Some("Esc"))
            .unwrap()
            .build(),
        &PredefinedMenuItem::separator(),
        &MenuItemBuilder::new()
            .text("Exit")
            .id(tray_icon::menu::MenuId::from("Exit"))
            .enabled(true)
            .accelerator(Some("Ctrl+Alt+Q"))
            .unwrap()
            .build(),
    ];

    let tray_menu = Menu::with_items(&menu_items).unwrap();

    let proxy_clone = proxy.clone();
    let proxy_clone2 = proxy.clone();
    tray_icon::TrayIconEvent::set_event_handler(Some(move |event| {
        proxy_clone
            .send_event(UserEvent::TrayIconEvent(event))
            .unwrap();
    }));

    tray_icon::menu::MenuEvent::set_event_handler(Some(move |event| {
        proxy_clone2
            .send_event(UserEvent::MenuEvent(event))
            .unwrap();
    }));

    // Embed the icon file
    let icon_data: &[u8] = include_bytes!("../assets/icon.ico");

    let icon_image = ico::IconDir::read(std::io::Cursor::new(icon_data))
        .expect("Failed to read icon")
        .entries()[0]
        .decode()
        .expect("Failed to decode icon")
        .rgba_data()
        .to_vec();

    // Create the icon from the embedded data
    let icon = Icon::from_rgba(icon_image, 32, 32).expect("Failed to create icon");

    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("SketchOver")
        .with_icon(icon)
        .build()
        .unwrap();
    tray_icon
}
