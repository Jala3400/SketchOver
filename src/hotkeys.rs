use crate::app::UserEvent;
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};

struct HotkeyAction {
    name: String,
    hotkey: HotKey,
    hotkey_event: HotkeyEvent,
    id: u32,
}

pub enum HotkeyEvent {
    Show,
    ShowPreserve,
}

impl HotkeyAction {
    fn new(name: String, hotkey: HotKey, hotkey_event: HotkeyEvent) -> Self {
        Self {
            name,
            hotkey,
            hotkey_event,
            id: hotkey.id(),
        }
    }
}

pub struct HotkeyManager {
    manager: GlobalHotKeyManager,
    hotkeys: Vec<HotkeyAction>,
}

impl HotkeyManager {
    pub fn new(proxy: &winit::event_loop::EventLoopProxy<UserEvent>) -> Self {
        let manager = GlobalHotKeyManager::new().unwrap();
        let mut hotkeys = Vec::with_capacity(1);

        // Define all hotkeys
        let possible_hotkeys = vec![
            HotkeyAction::new(
                "Show".to_string(),
                HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyS),
                HotkeyEvent::Show,
            ),
            HotkeyAction::new(
                "ShowPreserved".to_string(),
                HotKey::new(
                    Some(Modifiers::CONTROL | Modifiers::ALT | Modifiers::SHIFT),
                    Code::KeyS,
                ),
                HotkeyEvent::ShowPreserve,
            ),
        ];

        // Register all hotkeys
        for hotkey in possible_hotkeys {
            if let Ok(()) = manager.register(hotkey.hotkey.clone()) {
                hotkeys.push(hotkey);
            } else {
                eprintln!("Failed to register hotkey: {}", hotkey.name);
            }
        }

        let proxy_clone = proxy.clone();
        GlobalHotKeyEvent::set_event_handler(Some(move |event: GlobalHotKeyEvent| {
            proxy_clone
                .send_event(UserEvent::HotkeyEvent(event))
                .unwrap();
        }));

        Self { manager, hotkeys }
    }

    pub fn get_hotkey_event(&self, event: &GlobalHotKeyEvent) -> &HotkeyEvent {
        &self
            .hotkeys
            .iter()
            .find(|hotkey| hotkey.id == event.id())
            .unwrap()
            .hotkey_event
    }

    pub fn unregister_all(&self) {
        for hotkey in &self.hotkeys {
            let _ = self.manager.unregister(hotkey.hotkey);
        }
    }
}
