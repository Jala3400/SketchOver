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
    NewCanvas,
    ShowPrevious,
    TransparentToMouse,
    EscTransparentMouse,
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
        let mut hotkeys = Vec::with_capacity(4);

        // Added here because it is only activated when transparent to mouse is active
        hotkeys.push(HotkeyAction::new(
            "Hide".to_string(),
            HotKey::new(None, Code::Escape),
            HotkeyEvent::EscTransparentMouse,
        ));

        // Define all hotkeys
        let possible_hotkeys = vec![
            HotkeyAction::new(
                "New canvas".to_string(),
                HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyS),
                HotkeyEvent::NewCanvas,
            ),
            HotkeyAction::new(
                "Show previous".to_string(),
                HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyR),
                HotkeyEvent::ShowPrevious,
            ),
            HotkeyAction::new(
                "Transparent to mouse".to_string(),
                HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyT),
                HotkeyEvent::TransparentToMouse,
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

    pub fn setup_escape_transparent_mouse(&self) -> bool {
        let escape_key = self
            .hotkeys
            .iter()
            .find(|h| matches!(h.hotkey_event, HotkeyEvent::EscTransparentMouse))
            .map(|h| &h.hotkey)
            .unwrap();

        self.manager.register(escape_key.clone()).is_ok()
    }

    pub fn escape_transparent_to_mouse(&self) -> bool {
        let escape_key = self
            .hotkeys
            .iter()
            .find(|h| matches!(h.hotkey_event, HotkeyEvent::EscTransparentMouse))
            .map(|h| &h.hotkey)
            .unwrap();

        self.manager.unregister(escape_key.clone()).is_ok()
    }
}
