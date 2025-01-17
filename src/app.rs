use pixels::{wgpu, Pixels, PixelsBuilder, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, MouseButton, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes, WindowId},
};

#[derive(Default)]
pub struct App {
    pub window: Option<Window>,
    pub pixels: Option<Pixels>,
    pub attributes: WindowAttributes,
    pub cursor_pos: (f64, f64),
    pub window_size: (u32, u32),
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
            ..Default::default()
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

impl ApplicationHandler<UserEvent> for App {
    // This is a common indicator that you can create a window.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(self.attributes.clone()).unwrap());

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
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                ..
            } => {
                event_loop.exit();
            }
            WindowEvent::CursorMoved { position, .. } => {
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
                            let (x, y) = self.cursor_pos;
                            self.paint_pixel(x, y);
                            self.window.as_ref().unwrap().request_redraw();
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
}
