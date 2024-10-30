use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    event::{self, Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

const WIDTH: u32 = 880;
const HEIGHT: u32 = 480;

struct App {
    window: Window,
    pixels: Pixels,
    cursor_pos: (f64, f64),
}

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("My Window")
        .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT))
        .build(&event_loop)
        .unwrap();

    let pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut app = App {
        window,
        pixels,
        cursor_pos: (0.0, 0.0),
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
                                change_pixel_color(&mut app, x, y);
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

fn change_pixel_color(app: &mut App, x: f64, y: f64) {
    let window_size = app.window.inner_size();
    let index = ((y as u32 * window_size.width + x as u32) * 4) as usize;
    let frame = app.pixels.frame_mut();
    frame[index] = 255; // Red
    frame[index + 1] = 0; // Green
    frame[index + 2] = 0; // Blue
    frame[index + 3] = 255; // Alpha
}
