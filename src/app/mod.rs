use crate::{
    canvas::Canvas,
    cursor::{color_circle_cursor, erasing_cursor},
    hotkeys::HotkeyManager,
    tray_icon::setup_tray_icon,
};
use global_hotkey::GlobalHotKeyEvent;
use mouse_position::mouse_position::Mouse;
use std::rc::Rc;
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

#[derive(Debug, Clone, Copy)]
// ARGB format
pub enum Colors {
    RED = 0xffff0000,
    GREEN = 0xff00ff00,
    BLUE = 0xff0000ff,
    YELLOW = 0xffffff00,
    CYAN = 0xff00ffff,
    MAGENTA = 0xffff00ff,
    WHITE = 0xffffffff,
    BLACK = 0xff000000,
    TRANSPARENT = 0x00000000,
}

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Drawing(Colors),
    Erasing(Colors),
}

pub struct App {
    window: Option<Rc<Window>>,
    canvas: Option<Canvas>,
    _tray_icon: tray_icon::TrayIcon,
    hotkey_manager: HotkeyManager,
    _proxy: EventLoopProxy<UserEvent>,
    attributes: WindowAttributes,
    cursor_pos: (i32, i32),
    window_size: (i32, i32),
    is_clicked: bool,
    modifiers: winit::keyboard::ModifiersState,
    last_paint_time: std::time::Instant,
}

impl App {
    pub fn new(proxy: EventLoopProxy<UserEvent>) -> Self {
        App {
            window: None,
            canvas: None,
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
            cursor_pos: (0, 0),
            window_size: (0, 0),
            is_clicked: false,
            modifiers: winit::keyboard::ModifiersState::empty(),
            last_paint_time: std::time::Instant::now(),
        }
    }

    fn update_circle_cursor(&self, event_loop: &ActiveEventLoop) {
        if let Some(canvas) = &self.canvas {
            let mode = canvas.get_mode();
            let radius = canvas.get_radius();
            if let Mode::Drawing(color) = mode {
                self.window
                    .as_ref()
                    .unwrap()
                    .set_cursor(winit::window::Cursor::Custom(color_circle_cursor(
                        event_loop,
                        radius as i32,
                        color as u32,
                    )));
            } else {
                self.window
                    .as_ref()
                    .unwrap()
                    .set_cursor(winit::window::Cursor::Custom(erasing_cursor(
                        event_loop,
                        radius as i32,
                    )));
            }
        }
    }

    fn resize_radius(&mut self, event_loop: &ActiveEventLoop, delta: f64) {
        self.canvas.as_mut().unwrap().resize_radius(delta);
        self.update_circle_cursor(event_loop);
    }

    fn set_mode(&mut self, event_loop: &ActiveEventLoop, mode: Mode) {
        self.canvas.as_mut().unwrap().set_mode(mode);
        self.update_circle_cursor(event_loop);
    }

    fn toggle_mode(&mut self, event_loop: &ActiveEventLoop) {
        self.canvas.as_mut().unwrap().toggle_mode();
        self.update_circle_cursor(event_loop);
    }

    fn set_backgroudn_color(&mut self, color: Colors) {
        self.canvas.as_mut().unwrap().set_background_color(color);
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
