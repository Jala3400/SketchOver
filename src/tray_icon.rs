use tray_icon::{
    menu::{IsMenuItem, Menu, MenuItemBuilder, PredefinedMenuItem},
    TrayIconBuilder,
};
use winit::event_loop::EventLoopProxy;

use crate::app::UserEvent;

pub fn setup_tray_icon(proxy: &EventLoopProxy<UserEvent>) -> tray_icon::TrayIcon {
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

    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("SketchOver")
        .with_icon(tray_icon::Icon::from_path("assets/icon.ico", Some((32, 32))).unwrap())
        .build()
        .unwrap();
    tray_icon
}
