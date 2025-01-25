use super::{App, Colors, Mode};
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
                    match self.modifiers {
                        val if val == ModifiersState::CONTROL | ModifiersState::SHIFT => {
                            match key_code {
                                KeyCode::KeyQ => {
                                    event_loop.exit();
                                }
                                _ => (),
                            }
                        }

                        ModifiersState::CONTROL => match key_code {
                            KeyCode::KeyR => {
                                self.set_backgroudn_color(Colors::RED);
                                self.window.as_ref().unwrap().request_redraw();
                            }

                            KeyCode::KeyG => {
                                self.set_backgroudn_color(Colors::GREEN);
                                self.window.as_ref().unwrap().request_redraw();
                            }

                            KeyCode::KeyB => {
                                self.set_backgroudn_color(Colors::BLUE);
                                self.window.as_ref().unwrap().request_redraw();
                            }

                            KeyCode::KeyY => {
                                self.set_backgroudn_color(Colors::YELLOW);
                                self.window.as_ref().unwrap().request_redraw();
                            }

                            KeyCode::KeyC => {
                                self.set_backgroudn_color(Colors::CYAN);
                                self.window.as_ref().unwrap().request_redraw();
                            }

                            KeyCode::KeyM => {
                                self.set_backgroudn_color(Colors::MAGENTA);
                                self.window.as_ref().unwrap().request_redraw();
                            }

                            KeyCode::KeyW => {
                                self.set_backgroudn_color(Colors::WHITE);
                                self.window.as_ref().unwrap().request_redraw();
                            }

                            KeyCode::KeyK => {
                                self.set_backgroudn_color(Colors::BLACK);
                                self.window.as_ref().unwrap().request_redraw();
                            }

                            KeyCode::Space => {
                                self.set_backgroudn_color(Colors::TRANSPARENT);
                                self.window.as_ref().unwrap().request_redraw();
                            }

                            _ => (),
                        },

                        _ => {
                            match key_code {
                                KeyCode::Escape => {
                                    self.window.as_ref().unwrap().set_visible(false);
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
                                    self.set_mode(event_loop, Mode::Drawing(Colors::RED));
                                }

                                KeyCode::KeyG => {
                                    self.set_mode(event_loop, Mode::Drawing(Colors::GREEN));
                                }

                                KeyCode::KeyB => {
                                    self.set_mode(event_loop, Mode::Drawing(Colors::BLUE));
                                }

                                KeyCode::KeyY => {
                                    self.set_mode(event_loop, Mode::Drawing(Colors::YELLOW));
                                }

                                KeyCode::KeyC => {
                                    self.set_mode(event_loop, Mode::Drawing(Colors::CYAN));
                                }

                                KeyCode::KeyM => {
                                    self.set_mode(event_loop, Mode::Drawing(Colors::MAGENTA));
                                }

                                KeyCode::KeyW => {
                                    self.set_mode(event_loop, Mode::Drawing(Colors::WHITE));
                                }

                                KeyCode::KeyK => {
                                    self.set_mode(event_loop, Mode::Drawing(Colors::BLACK));
                                }

                                KeyCode::Space => {
                                    self.toggle_mode(event_loop);
                                }

                                _ => (),
                            }
                        }
                    }
                }
            }

            WindowEvent::CursorMoved { position, .. } => {
                if self.is_clicked {
                    let now = std::time::Instant::now();
                    let elapsed = now - self.last_paint_time;

                    if elapsed.as_millis() >= 7 {
                        self.canvas.as_mut().unwrap().paint_line(
                            self.cursor_pos.0,
                            self.cursor_pos.1,
                            position.x as i32,
                            position.y as i32,
                            self.radius as i32,
                        );
                        self.window.as_ref().unwrap().request_redraw();
                        self.last_paint_time = now;
                        self.cursor_pos = (position.x as i32, position.y as i32);
                    }
                } else {
                    self.cursor_pos = (position.x as i32, position.y as i32);
                }
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
                            self.canvas.as_mut().unwrap().paint_circle(
                                self.cursor_pos.0 as i32,
                                self.cursor_pos.1 as i32,
                                self.radius as i32,
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
