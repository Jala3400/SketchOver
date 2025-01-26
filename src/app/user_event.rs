use crate::hotkeys::HotkeyEvent;

use super::{App, UserEvent};
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
                    self.show_window();
                }
                _ => (),
            },
            UserEvent::MenuEvent(event) => match event.id.0.as_str() {
                "Show" => {
                    self.show_window();
                }
                "Hide" => {
                    self.hide_window();
                }
                "Exit" => {
                    _event_loop.exit();
                }
                _ => (),
            },
            UserEvent::HotkeyEvent(event) => {
                // First get the action name without borrowing self
                let hotkey_event = self.hotkey_manager.get_hotkey_event(&event);

                // Then execute the action
                match hotkey_event {
                    HotkeyEvent::Show => {
                        self.canvas.as_mut().unwrap().clear();
                        self.show_window_in_current_monitor();
                    }
                    HotkeyEvent::ShowPreserve => {
                        self.show_window_in_current_monitor();
                    }
                }
            }
        }
    }
}
