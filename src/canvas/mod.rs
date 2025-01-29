use crate::app::{Colors, Mode};
use softbuffer::Surface;
use std::rc::Rc;
use winit::window::Window;

mod drawings;
mod surface;

pub struct Canvas {
    drawing: Vec<u32>,
    surface: Surface<Rc<Window>, Rc<Window>>,
    radius: f64, // It is needed as f64 to be able to change the size
    mode: Mode,
    background_color: Colors,
    window_size: (i32, i32),
}

impl Canvas {
    pub fn new(surface: Surface<Rc<Window>, Rc<Window>>, window_size: (i32, i32)) -> Canvas {
        Canvas {
            drawing: vec![0; (window_size.0 * window_size.1) as usize],
            surface: surface,
            radius: 2.0,
            mode: Mode::Drawing(Colors::RED),
            background_color: Colors::TRANSPARENT,
            window_size,
        }
    }
}
