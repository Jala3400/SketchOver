use crate::hotkeys::HotkeyEvent;

use super::{App, UserEvent};
use global_hotkey::HotKeyState;
use winit::event_loop::ActiveEventLoop;

impl App {
    pub fn user_event_func(&mut self, event_loop: &ActiveEventLoop, event: UserEvent) {
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
                "Transparent to mouse" => {
                    self.toggle_transparent_to_mouse();
                }
                "New canvas" => {
                    self.show_new_window(event_loop);
                }
                "Show previous" => {
                    self.show_window();
                }
                "Hide" => {
                    self.hide_window();
                }
                "Exit" => {
                    event_loop.exit();
                }
                _ => (),
            },
            UserEvent::HotkeyEvent(event) => {
                if event.state == HotKeyState::Pressed {
                    // First get the action name without borrowing self
                    let hotkey_event = self.hotkey_manager.get_hotkey_event(&event);

                    // Then execute the action
                    match hotkey_event {
                        HotkeyEvent::ShowNew => {
                            self.show_new_window_in_current_monitor(event_loop);
                        }
                        HotkeyEvent::ShowPrevious => {
                            self.show_window_in_current_monitor();
                        }
                        HotkeyEvent::TransparentToMouse => {
                            self.toggle_transparent_to_mouse();
                        }
                    }
                }
            }
        }
    }
}
