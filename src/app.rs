use pixels::{wgpu, Pixels, PixelsBuilder, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, MouseButton, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, ModifiersState, PhysicalKey},
    monitor::MonitorHandle,
    window::{Window, WindowAttributes, WindowId},
};

#[derive(Default)]
pub struct App {
    window: Option<Window>,
    pixels: Option<Pixels>,
    attributes: WindowAttributes,
    cursor_pos: (f64, f64),
    window_size: (u32, u32),
    radius: f64,
    is_clicked: bool,
    modifiers: winit::keyboard::ModifiersState,
}

#[derive(Debug)]
pub enum UserEvent {
    TrayIconEvent(tray_icon::TrayIconEvent),
    MenuEvent(tray_icon::menu::MenuEvent),
}

impl App {
    pub fn new(attributes: WindowAttributes) -> Self {
        App {
            attributes,
            radius: 2.0,
            ..Default::default()
        }
    }

    fn paint_line(&mut self, x0: f64, y0: f64, x1: f64, y1: f64) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();

        let sx = if x0 < x1 { 1.0 } else { -1.0 };
        let sy = if y0 < y1 { 1.0 } else { -1.0 };

        let mut err = dx - dy;
        let mut x = x0;
        let mut y = y0;

        while x != x1 || y != y1 {
            self.paint_circle(x, y);

            let e2 = 2.0 * err;

            if e2 > -dy {
                err -= dy;
                x += sx;
            }

            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
        self.paint_circle(x, y);
    }

    fn paint_circle(&mut self, center_x: f64, center_y: f64) {
        let radius = self.radius;
        let mut x = radius;
        let mut y = 0.0;
        let mut decision = 1.0 - radius;

        while x >= y {
            // Draw horizontal lines to fill the circle
            for i in (-x as i32)..=(x as i32) {
                self.paint_pixel(center_x + i as f64, center_y + y);
                self.paint_pixel(center_x + i as f64, center_y - y);
            }
            for i in (-y as i32)..=(y as i32) {
                self.paint_pixel(center_x + i as f64, center_y + x);
                self.paint_pixel(center_x + i as f64, center_y - x);
            }

            y += 1.0;

            if decision <= 0.0 {
                decision += 2.0 * y + 1.0;
            } else {
                x -= 1.0;
                decision += 2.0 * (y - x) + 1.0;
            }
        }
    }

    fn paint_pixel(&mut self, x: f64, y: f64) {
        let (width, height) = self.window_size;

        // Bounds checking
        if x < 0.0 || y < 0.0 || x >= width as f64 || y >= height as f64 {
            return;
        }

        let index = ((y as u32 * width + x as u32) * 4) as usize;
        let frame = self.pixels.as_mut().unwrap().frame_mut();

        // Write all color components at once using array slice
        frame[index..index + 4].copy_from_slice(&[255, 0, 0, 255]);
    }
}

impl App {
    pub fn update_circle_cursor(&mut self, event_loop: &ActiveEventLoop) {
        let radius = self.radius as i32;
        let diameter = radius * 2 + 1 as i32;
        // Using a flat vector to store the pattern
        let mut cursor_pattern = vec![0u8; ((diameter * diameter) * 4) as usize];
        let hotspot_x = radius as u16;
        let hotspot_y = radius as u16;

        let mut x = radius;
        let mut y = 0;
        let mut decision = 1 - radius;

        while x >= y {
            // Draw horizontal lines to fill the circle
            for i in (-x as i32)..=(x as i32) {
                let center = radius as i32;
                App::set_pixel(&mut cursor_pattern, center + i, center + y, diameter);
                App::set_pixel(&mut cursor_pattern, center + i, center - y, diameter);
            }
            for i in (-y as i32)..=(y as i32) {
                let center = radius as i32;
                App::set_pixel(&mut cursor_pattern, center + i, center + x, diameter);
                App::set_pixel(&mut cursor_pattern, center + i, center - x, diameter);
            }

            y += 1;

            if decision <= 0 {
                decision += 2 * y + 1;
            } else {
                x -= 1;
                decision += 2 * (y - x) + 1;
            }
        }

        let custom_cursor_source = winit::window::CustomCursor::from_rgba(
            cursor_pattern,
            diameter as u16,
            diameter as u16,
            hotspot_x,
            hotspot_y,
        )
        .unwrap();

        self.window
            .as_ref()
            .unwrap()
            .set_cursor(winit::window::Cursor::Custom(
                event_loop.create_custom_cursor(custom_cursor_source),
            ));
    }

    fn set_pixel(cursor_pattern: &mut Vec<u8>, x: i32, y: i32, diameter: i32) {
        let idx = ((y * diameter + x) * 4) as usize;
        cursor_pattern[idx] = 255; // R
        cursor_pattern[idx + 1] = 0; // G
        cursor_pattern[idx + 2] = 0; // B
        cursor_pattern[idx + 3] = 255; // A
    }
}

impl ApplicationHandler<UserEvent> for App {
    // This is a common indicator that you can create a window.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(self.attributes.clone()).unwrap());
        self.update_circle_cursor(event_loop);

        let window_phisical = self.window.as_ref().unwrap().inner_size();
        self.window_size = (window_phisical.width, window_phisical.height);

        let surface_texture = SurfaceTexture::new(
            self.window_size.0,
            self.window_size.1,
            self.window.as_ref().unwrap(),
        );

        self.pixels = Some(
            PixelsBuilder::new(self.window_size.0, self.window_size.1, surface_texture)
                .surface_texture_format(wgpu::TextureFormat::Bgra8UnormSrgb)
                .clear_color(wgpu::Color::TRANSPARENT)
                .blend_state(wgpu::BlendState::ALPHA_BLENDING)
                .build()
                .unwrap(),
        );
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        // `unwrap` is fine, the window will always be available when
        // receiving a window event.
        let window = self.window.as_ref().unwrap();
        // Handle window event.

        match event {
            WindowEvent::ModifiersChanged(modifiers) => {
                self.modifiers = modifiers.state();
            }

            WindowEvent::MouseWheel { delta, .. } => {
                let delta_value = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => y as f64,
                    winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as f64 / 50.0,
                };
                self.radius = (self.radius + delta_value).max(1.0).min(20.0);
                self.update_circle_cursor(event_loop);
            }

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key_code),
                        state,
                        ..
                    },
                ..
            } => {
                if state == ElementState::Pressed {
                    match key_code {
                        KeyCode::Escape => {
                            self.window.as_ref().unwrap().set_visible(false);
                        }

                        KeyCode::KeyQ => {
                            if self.modifiers == ModifiersState::CONTROL | ModifiersState::SHIFT {
                                event_loop.exit();
                            }
                        }

                        KeyCode::Tab => {
                            let current_monitor = window
                                .current_monitor()
                                .unwrap_or_else(|| window.primary_monitor().unwrap());
                            let all_monitors: Vec<MonitorHandle> =
                                window.available_monitors().collect();

                            if !all_monitors.is_empty() {
                                // Find current monitor index
                                let current_idx = all_monitors
                                    .iter()
                                    .position(|m| m.name() == current_monitor.name())
                                    .unwrap_or(0);

                                // Get next monitor (wrap around to first if at end)
                                let next_idx = (current_idx + 1) % all_monitors.len();
                                let next_monitor = &all_monitors[next_idx];

                                // First move to the desired monitor, then maximize
                                window.set_outer_position(winit::dpi::PhysicalPosition::new(
                                    next_monitor.position().x,
                                    next_monitor.position().y,
                                ));
                                window.set_maximized(true);
                            }
                        }
                        _ => (),
                    }
                }
            }

            WindowEvent::CursorMoved { position, .. } => {
                if self.is_clicked {
                    self.paint_line(self.cursor_pos.0, self.cursor_pos.1, position.x, position.y);
                    self.window.as_ref().unwrap().request_redraw();
                }
                self.cursor_pos = (position.x, position.y);
            }

            WindowEvent::Resized(size) => {
                let pixels = self.pixels.as_mut().unwrap();
                let _ = pixels.resize_surface(size.width, size.height);
                let _ = pixels.resize_buffer(size.width, size.height);
                window.request_redraw();
            }

            WindowEvent::MouseInput { state, button, .. } => {
                if state == ElementState::Pressed {
                    match button {
                        MouseButton::Left => {
                            self.is_clicked = true;
                            self.paint_circle(self.cursor_pos.0, self.cursor_pos.1);
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

            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::RedrawRequested => {
                self.pixels.as_ref().unwrap().render().unwrap();
            }
            _ => (),
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: UserEvent) {
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
        }
    }
}
