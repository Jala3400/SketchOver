use super::{App, Colors, Mode};
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
                                _ => (),
                            }
                        }

                        ModifiersState::CONTROL => match key_code {
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

                            KeyCode::KeyK => {
                                self.set_backgroudn_color(Colors::BLACK);
                            }

                            KeyCode::Space => {
                                self.set_backgroudn_color(Colors::TRANSPARENT);
                            }

                            _ => (),
                        },

                        _ => match key_code {
                            KeyCode::Escape => {
                                self.set_window_visibility(false);
                            }

                            KeyCode::Tab => {
                                self.cycle_through_monitors();
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
                        },
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
