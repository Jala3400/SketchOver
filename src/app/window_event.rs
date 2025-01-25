use super::{App, Colors};
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
                    winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y / 50.0,
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

                                self.assign_monitor(next_monitor);
                            }
                        }

                        KeyCode::KeyR => {
                            self.set_mode(event_loop, super::Mode::Drawing(Colors::RED));
                        }

                        KeyCode::KeyG => {
                            self.set_mode(event_loop, super::Mode::Drawing(Colors::GREEN));
                        }

                        KeyCode::KeyB => {
                            self.set_mode(event_loop, super::Mode::Drawing(Colors::BLUE));
                        }

                        KeyCode::KeyY => {
                            self.set_mode(event_loop, super::Mode::Drawing(Colors::YELLOW));
                        }

                        KeyCode::KeyC => {
                            self.set_mode(event_loop, super::Mode::Drawing(Colors::CYAN));
                        }

                        KeyCode::KeyM => {
                            self.set_mode(event_loop, super::Mode::Drawing(Colors::MAGENTA));
                        }

                        KeyCode::KeyW => {
                            self.set_mode(event_loop, super::Mode::Drawing(Colors::WHITE));
                        }

                        KeyCode::KeyK => {
                            self.set_mode(event_loop, super::Mode::Drawing(Colors::BLACK));
                        }

                        KeyCode::Space => {
                            self.toggle_mode(event_loop);
                        }

                        _ => (),
                    }
                }
            }

            WindowEvent::CursorMoved { position, .. } => {
                if self.is_clicked {
                    let color = self.get_drawing_color();
                    self.canvas.as_mut().unwrap().paint_line(
                        self.cursor_pos.0,
                        self.cursor_pos.1,
                        position.x as i32,
                        position.y as i32,
                        self.radius as i32,
                        color,
                    );
                    self.window.as_ref().unwrap().request_redraw();
                }
                self.cursor_pos = (position.x as i32, position.y as i32);
            }

            WindowEvent::Resized(size) => {
                self.canvas
                    .as_mut()
                    .unwrap()
                    .resize(size.width as i32, size.height as i32);
                window.request_redraw();
            }

            WindowEvent::MouseInput { state, button, .. } => {
                if state == ElementState::Pressed {
                    match button {
                        MouseButton::Left => {
                            self.is_clicked = true;
                            let color = self.get_drawing_color();
                            self.canvas.as_mut().unwrap().paint_circle(
                                self.cursor_pos.0 as i32,
                                self.cursor_pos.1 as i32,
                                self.radius as i32,
                                color,
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
                self.canvas.as_mut().unwrap().redraw();
            }
            _ => (),
        }
    }
}
