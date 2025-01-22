use crate::app::UserEvent;
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};

pub fn setup_hotkeys(proxy: &winit::event_loop::EventLoopProxy<UserEvent>) -> GlobalHotKeyManager {
    let manager = GlobalHotKeyManager::new().unwrap();

    let hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyS);
    if let Err(e) = manager.register(hotkey) {
        eprintln!("Failed to register hotkey: {}", e);
        // Optionally, try a different hotkey combination or inform the user
    }

    let proxy_clone = proxy.clone();

    GlobalHotKeyEvent::set_event_handler(Some(move |event: GlobalHotKeyEvent| {
        proxy_clone
            .send_event(UserEvent::HotkeyEvent(event))
            .unwrap();
    }));

    manager
}
