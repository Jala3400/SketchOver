use crate::{canvas::Canvas, cursor::custom_circle_cursor, hotkeys::HotkeyManager};
use global_hotkey::GlobalHotKeyEvent;
use mouse_position::mouse_position::Mouse;
use pixels::Pixels;
use winit::{
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes},
};
mod app_handler;
mod resumed;
mod user_event;
mod window_event;

#[derive(Debug)]
pub enum UserEvent {
    TrayIconEvent(tray_icon::TrayIconEvent),
    MenuEvent(tray_icon::menu::MenuEvent),
    HotkeyEvent(GlobalHotKeyEvent),
}

#[derive(Default)]
pub struct App {
    window: Option<Window>,
    pixels: Option<Pixels>,
    canvas: Canvas,
    hotkey_manager: Option<HotkeyManager>,
    attributes: WindowAttributes,
    cursor_pos: (f64, f64),
    window_size: (u32, u32),
    radius: f64,
    is_clicked: bool,
    modifiers: winit::keyboard::ModifiersState,
}

impl App {
    pub fn new(attributes: WindowAttributes, hotkey_manager: HotkeyManager) -> Self {
        App {
            window: None,
            pixels: None,
            canvas: Canvas::default(),
            hotkey_manager: Some(hotkey_manager),
            attributes,
            cursor_pos: (0.0, 0.0),
            window_size: (0, 0),
            radius: 2.0,
            is_clicked: false,
            modifiers: winit::keyboard::ModifiersState::empty(),
        }
    }

    fn update_circle_cursor(&self, event_loop: &ActiveEventLoop) {
        self.window
            .as_ref()
            .unwrap()
            .set_cursor(winit::window::Cursor::Custom(custom_circle_cursor(
                event_loop,
                self.radius as i32,
            )));
    }

    pub fn set_window_visibility(&self, visible: bool) {
        if let Some(window) = &self.window {
            window.set_visible(visible);
        }
    }

    pub fn show_window_in_current_monitor(&self) {
        let position = Mouse::get_mouse_position();
        if let Mouse::Position { x, y } = position {
            self.set_window_monitor_from_cursor(x, y);
        }

        self.set_window_visibility(true);
    }

    pub fn set_window_monitor_from_cursor(&self, x: i32, y: i32) {
        if let Some(window) = &self.window {
            let monitor = window.available_monitors().find(|monitor| {
                let position = monitor.position();
                let size = monitor.size();
                x >= position.x
                    && x <= position.x + size.width as i32
                    && y >= position.y
                    && y <= position.y + size.height as i32
            });

            window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(monitor)));
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        if let Some(hotkey_manager) = &self.hotkey_manager {
            hotkey_manager.unregister_all();
        }
    }
}
