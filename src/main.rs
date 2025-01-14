use pixels::{wgpu, Error, Pixels, PixelsBuilder, SurfaceTexture};
use winit::{
    event::{self, Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

struct App {
    window: Window,
    pixels: Pixels,
    cursor_pos: (f64, f64),
    window_size: (u32, u32),
}

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("My Window")
        .with_transparent(true)
        .with_decorations(false)
        .with_maximized(true)
        .with_resizable(false)
        // .with_window_level(winit::window::WindowLevel::AlwaysOnTop)
        .build(&event_loop)
        .unwrap();

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

    let pixels = PixelsBuilder::new(window_size.width, window_size.height, surface_texture)
        .surface_texture_format(wgpu::TextureFormat::Bgra8UnormSrgb)
        .clear_color(wgpu::Color::TRANSPARENT)
        .blend_state(wgpu::BlendState::ALPHA_BLENDING)
        .build()?;

    let mut app = App {
        window,
        pixels,
        cursor_pos: (0.0, 0.0),
        window_size: (window_size.width, window_size.height),
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                    Some(event::VirtualKeyCode::Escape) => {
                        *control_flow = winit::event_loop::ControlFlow::Exit
                    }
                    _ => (),
                },
                WindowEvent::CursorMoved { position, .. } => {
                    app.cursor_pos = (position.x, position.y);
                }
                WindowEvent::Resized(size) => {
                    let _ = app.pixels.resize_surface(size.width, size.height);
                    let _ = app.pixels.resize_buffer(size.width, size.height);
                    app.window.request_redraw();
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    if state == winit::event::ElementState::Pressed {
                        match button {
                            winit::event::MouseButton::Left => {
                                let (x, y) = app.cursor_pos;
                                paint_pixel(&mut app, x, y);
                                app.window.request_redraw();
                            }
                            _ => (),
                        }
                    }
                }
                WindowEvent::CloseRequested => *control_flow = winit::event_loop::ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                app.pixels.render().unwrap();
            }
            _ => (),
        }
    });
}

fn paint_pixel(app: &mut App, x: f64, y: f64) {
    let (width, height) = app.window_size;

    // Bounds checking
    if x < 0.0 || y < 0.0 || x >= width as f64 || y >= height as f64 {
        return;
    }

    let index = ((y as u32 * width + x as u32) * 4) as usize;
    let frame = app.pixels.frame_mut();

    // Write all color components at once using array slice
    frame[index..index + 4].copy_from_slice(&[255, 0, 0, 255]);
}
