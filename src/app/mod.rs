use crate::{canvas::Canvas, cursor::custom_circle_cursor};
use global_hotkey::GlobalHotKeyEvent;
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
    attributes: WindowAttributes,
    cursor_pos: (f64, f64),
    window_size: (u32, u32),
    radius: f64,
    is_clicked: bool,
    modifiers: winit::keyboard::ModifiersState,
}

impl App {
    pub fn new(attributes: WindowAttributes) -> Self {
        App {
            attributes,
            radius: 2.0,
            ..Default::default()
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
}
