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
    event::{ElementState, MouseButton},
    event_loop::{ActiveEventLoop, EventLoopProxy},
    monitor::MonitorHandle,
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
#[repr(u32)]
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
    is_clicked: bool,
    tab_pressed: bool,
    transparent_to_mouse: bool,
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
            is_clicked: false,
            tab_pressed: false,
            transparent_to_mouse: false,
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
        self.window.as_ref().unwrap().request_redraw();
    }

    pub fn clear_canvas(&mut self) {
        self.canvas.as_mut().unwrap().clear();
        self.window.as_ref().unwrap().request_redraw();
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.canvas
            .as_mut()
            .unwrap()
            .resize(width as i32, height as i32);
        self.window.as_ref().unwrap().request_redraw();
    }

    pub fn cursor_moved(&mut self, x: f64, y: f64) {
        let x = x as i32;
        let y = y as i32;
        if self.is_clicked {
            let now = std::time::Instant::now();
            let elapsed = now - self.last_paint_time;

            // Throttle paint
            if elapsed.as_millis() >= 7 {
                self.canvas.as_mut().unwrap().paint_line(
                    self.cursor_pos.0,
                    self.cursor_pos.1,
                    x,
                    y,
                );
                self.window.as_ref().unwrap().request_redraw();
                self.last_paint_time = now;
                self.cursor_pos = (x, y);
            }
        } else {
            self.cursor_pos = (x, y);
        }
    }

    pub fn mouse_pressed(&mut self, state: ElementState, button: MouseButton) {
        if state == ElementState::Pressed {
            match button {
                MouseButton::Left => {
                    self.is_clicked = true;
                    self.canvas
                        .as_mut()
                        .unwrap()
                        .paint_circle(self.cursor_pos.0 as i32, self.cursor_pos.1 as i32);
                    self.window.as_ref().unwrap().request_redraw();
                }
                _ => (),
            }
        } else {
            match button {
                MouseButton::Left => {
                    self.is_clicked = false;
                }
                _ => (),
            }
        }
    }

    fn reset(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(canvas) = &mut self.canvas {
            canvas.reset();
            canvas.redraw();
            self.update_circle_cursor(event_loop);
        }
    }

    fn show_new_window(&mut self, event_loop: &ActiveEventLoop) {
        self.reset(event_loop);
        self.show_window();
    }

    fn show_window(&mut self) {
        if self.transparent_to_mouse {
            self.toggle_transparent_to_mouse();
        }
        self.set_window_visibility(true);
        self.window.as_ref().unwrap().focus_window();
    }

    fn hide_window(&self) {
        self.set_window_visibility(false);
    }

    pub fn set_window_visibility(&self, visible: bool) {
        if let Some(window) = &self.window {
            window.set_visible(visible);
        }
    }

    pub fn cycle_through_monitors(&mut self) {
        if !self.tab_pressed {
            if let Some(window) = &self.window {
                let current_monitor = window
                    .current_monitor()
                    .unwrap_or_else(|| window.primary_monitor().unwrap());
                let all_monitors: Vec<MonitorHandle> = window.available_monitors().collect();

                if !all_monitors.is_empty() {
                    // Find current monitor index
                    let current_idx = all_monitors
                        .iter()
                        .position(|m| m.name() == current_monitor.name())
                        .unwrap_or(0);

                    // Get next monitor (wrap around to first if at end)
                    let next_idx = (current_idx + 1) % all_monitors.len();
                    let next_monitor = &all_monitors[next_idx];

                    self.assign_monitor(next_monitor);
                }
            }
        }
    }

    pub fn show_new_window_in_current_monitor(&mut self, event_loop: &ActiveEventLoop) {
        self.reset(event_loop);
        self.show_window_in_current_monitor();
    }

    pub fn show_window_in_current_monitor(&mut self) {
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

        self.show_window();
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

    fn toggle_transparent_to_mouse(&mut self) {
        if let (Some(window), Some(canvas)) = (self.window.as_ref(), self.canvas.as_mut()) {
            self.transparent_to_mouse = !self.transparent_to_mouse;

            // Set canvas opacity based on transparency
            canvas.set_opacity(if self.transparent_to_mouse { 64 } else { 255 });

            if !self.transparent_to_mouse {
                window.focus_window();
            }

            let _ = window.set_cursor_hittest(!self.transparent_to_mouse);
            window.request_redraw();
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        self.hotkey_manager.unregister_all();
    }
}
