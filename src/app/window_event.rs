use super::App;
use winit::{
    event::{ElementState, KeyEvent, MouseButton, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, ModifiersState, PhysicalKey},
    monitor::MonitorHandle,
    window::WindowId,
};

impl App {
    pub fn window_event_func(
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

            WindowEvent::Resized(_) => {
                self.canvas.clear();
                let size = window.inner_size();
                self.window_size = (size.width, size.height);
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
}
