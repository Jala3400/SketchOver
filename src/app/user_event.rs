use crate::hotkeys::HotkeyEvent;

use super::{App, UserEvent};
use mouse_position::mouse_position::Mouse;
use winit::event_loop::ActiveEventLoop;

impl App {
    pub fn user_event_func(&mut self, _event_loop: &ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::TrayIconEvent(_event) => match _event {
                tray_icon::TrayIconEvent::DoubleClick {
                    id: _,
                    position: _,
                    rect: _,
                    button: _,
                } => {
                    let window = self.window.as_ref().unwrap();
                    window.set_visible(!window.is_visible().unwrap_or(true));
                }
                _ => (),
            },
            UserEvent::MenuEvent(event) => match event.id.0.as_str() {
                "Show" => {
                    self.window.as_ref().unwrap().set_visible(true);
                }
                "Hide" => {
                    self.window.as_ref().unwrap().set_visible(false);
                }
                "Exit" => {
                    _event_loop.exit();
                }
                _ => (),
            },
            UserEvent::HotkeyEvent(event) => {
                // First get the action name without borrowing self
                let hotkey_event = self
                    .hotkey_manager
                    .as_ref()
                    .unwrap()
                    .get_hotkey_event(&event);

                // Then execute the action
                match hotkey_event {
                    HotkeyEvent::Show => {
                        let position = Mouse::get_mouse_position();
                        if let Mouse::Position { x, y } = position {
                            self.set_window_monitor_from_cursor(x, y);
                        }

                        self.set_window_visibility(true);
                    }
                }
            }
        }
    }
}
