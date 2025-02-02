use super::{App, Colors};
use winit::{
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, ModifiersState, PhysicalKey},
    window::WindowId,
};

impl App {
    pub fn window_event_func(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::ModifiersChanged(modifiers) => {
                self.modifiers = modifiers.state();
            }

            WindowEvent::MouseWheel { delta, .. } => {
                let delta_value = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => y as f64,
                    winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y / 50.0,
                };
                self.resize_radius(event_loop, delta_value);
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
                        val if val == ModifiersState::CONTROL | ModifiersState::ALT => {
                            match key_code {
                                KeyCode::KeyQ => {
                                    event_loop.exit();
                                }

                                KeyCode::KeyC => {
                                    self.clear_canvas();
                                }
                                _ => (),
                            }
                        }

                        ModifiersState::CONTROL => match key_code {
                            KeyCode::KeyZ => {
                                self.undo();
                            }

                            KeyCode::KeyR => {
                                self.set_backgroudn_color(Colors::RED);
                            }

                            KeyCode::KeyG => {
                                self.set_backgroudn_color(Colors::GREEN);
                            }

                            KeyCode::KeyB => {
                                self.set_backgroudn_color(Colors::BLUE);
                            }

                            KeyCode::KeyY => {
                                self.set_backgroudn_color(Colors::YELLOW);
                            }

                            KeyCode::KeyC => {
                                self.set_backgroudn_color(Colors::CYAN);
                            }

                            KeyCode::KeyM => {
                                self.set_backgroudn_color(Colors::MAGENTA);
                            }

                            KeyCode::KeyW => {
                                self.set_backgroudn_color(Colors::WHITE);
                            }

                            KeyCode::KeyK | KeyCode::KeyA => {
                                self.set_backgroudn_color(Colors::BLACK);
                            }

                            KeyCode::Space => {
                                self.set_backgroudn_color(Colors::TRANSPARENT);
                            }

                            _ => (),
                        },

                        _ => match key_code {
                            KeyCode::Escape => {
                                self.hide_window();
                            }

                            KeyCode::Tab => {
                                self.cycle_through_monitors();
                                self.tab_pressed = true;
                            }

                            KeyCode::KeyR => {
                                self.set_current_color(event_loop, Colors::RED);
                            }

                            KeyCode::KeyG => {
                                self.set_current_color(event_loop, Colors::GREEN);
                            }

                            KeyCode::KeyB => {
                                self.set_current_color(event_loop, Colors::BLUE);
                            }

                            KeyCode::KeyY => {
                                self.set_current_color(event_loop, Colors::YELLOW);
                            }

                            KeyCode::KeyC => {
                                self.set_current_color(event_loop, Colors::CYAN);
                            }

                            KeyCode::KeyM => {
                                self.set_current_color(event_loop, Colors::MAGENTA);
                            }

                            KeyCode::KeyW => {
                                self.set_current_color(event_loop, Colors::WHITE);
                            }

                            KeyCode::KeyK | KeyCode::KeyA => {
                                self.set_current_color(event_loop, Colors::BLACK);
                            }

                            KeyCode::Space => {
                                self.toggle_mode(event_loop);
                            }

                            _ => (),
                        },
                    }
                } else {
                    match key_code {
                        KeyCode::Tab => {
                            self.tab_pressed = false;
                        }

                        _ => (),
                    }
                }
            }

            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_moved(position.x, position.y);
            }
            WindowEvent::Resized(size) => {
                self.resize(size.width, size.height);
            }

            WindowEvent::MouseInput { state, button, .. } => {
                self.mouse_pressed(state, button);
            }

            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::RedrawRequested => {
                self.canvas.as_mut().unwrap().redraw();
            }
            _ => (),
        }
    }
}
