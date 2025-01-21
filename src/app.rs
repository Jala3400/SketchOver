use crate::{canvas::Canvas, cursor::custom_circle_cursor};
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
    canvas: Canvas,
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

impl App {}

impl ApplicationHandler<UserEvent> for App {
    // This is a common indicator that you can create a window.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(self.attributes.clone()).unwrap());

        self.update_circle_cursor(event_loop);

        let window_phisical = self.window.as_ref().unwrap().inner_size();
        self.window_size = (window_phisical.width, window_phisical.height);
        self.canvas = Canvas::new(self.window_size);

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
                    self.canvas.paint_line(
                        self.cursor_pos.0,
                        self.cursor_pos.1,
                        position.x,
                        position.y,
                        self.radius,
                    );
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
                            self.canvas.paint_circle(
                                self.cursor_pos.0,
                                self.cursor_pos.1,
                                self.radius,
                            );
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
                let pixels = self.pixels.as_mut().unwrap();
                let frame = pixels.frame_mut();

                // Copy canvas buffer into pixels frame
                frame.copy_from_slice(self.canvas.buffer());

                // Render the frame
                pixels.render().unwrap();
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
