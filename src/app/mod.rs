use crate::{
    canvas::Canvas, cursor::custom_circle_cursor, hotkeys::HotkeyManager,
    tray_icon::setup_tray_icon,
};
use global_hotkey::GlobalHotKeyEvent;
use mouse_position::mouse_position::Mouse;
use pixels::Pixels;
use winit::{
    event_loop::{ActiveEventLoop, EventLoopProxy},
    platform::windows::WindowAttributesExtWindows,
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

pub struct App {
    window: Option<Window>,
    pixels: Option<Pixels>,
    canvas: Canvas,
    _tray_icon: tray_icon::TrayIcon,
    hotkey_manager: HotkeyManager,
    _proxy: EventLoopProxy<UserEvent>,
    attributes: WindowAttributes,
    cursor_pos: (f64, f64),
    window_size: (u32, u32),
    radius: f64,
    is_clicked: bool,
    modifiers: winit::keyboard::ModifiersState,
}

impl App {
    pub fn new(proxy: EventLoopProxy<UserEvent>) -> Self {
        App {
            window: None,
            pixels: None,
            canvas: Canvas::default(),
            hotkey_manager: HotkeyManager::new(&proxy),
            _tray_icon: setup_tray_icon(&proxy),
            _proxy: proxy,
            attributes: WindowAttributes::default()
                .with_title("My Window")
                .with_transparent(true)
                .with_decorations(false)
                .with_resizable(false)
                .with_skip_taskbar(true)
                .with_visible(false)
                .with_window_level(winit::window::WindowLevel::AlwaysOnTop),
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
        if let Some(window) = &self.window {
            if let Mouse::Position { x, y } = position {
                let monitor = window.available_monitors().find(|monitor| {
                    let position = monitor.position();
                    let size = monitor.size();
                    x >= position.x
                        && x <= position.x + size.width as i32
                        && y >= position.y
                        && y <= position.y + size.height as i32
                });

                if let Some(monitor) = monitor {
                    self.assign_monitor(&monitor);
                }
            }
        }

        self.set_window_visibility(true);
    }

    fn assign_monitor(&self, monitor: &winit::monitor::MonitorHandle) {
        let monitor_pos = monitor.position();
        let work_area = monitor.size();

        self.window
            .as_ref()
            .unwrap()
            .set_outer_position(winit::dpi::PhysicalPosition::new(
                monitor_pos.x + 1,
                monitor_pos.y + 1,
            ));
        let _ = self
            .window
            .as_ref()
            .unwrap()
            .request_inner_size(winit::dpi::PhysicalSize::new(
                work_area.width - 2,
                work_area.height - 2,
            ));
    }
}

impl Drop for App {
    fn drop(&mut self) {
        self.hotkey_manager.unregister_all();
    }
}
