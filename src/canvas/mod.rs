use crate::app::{Colors, Mode};
use softbuffer::Surface;
use std::rc::Rc;
use winit::window::Window;

mod drawings;
mod mouse_handler;
mod preview;
mod surface;
mod undo;

#[derive(PartialEq)]
// It is either none or they contain the coordinates where it was clicked
pub enum Preview {
    Line(i32, i32),
    Square(i32, i32),
    None,
}

pub struct Canvas {
    drawing: Vec<u32>,
    surface: Surface<Rc<Window>, Rc<Window>>,
    history : Vec<Vec<u32>>,
    max_history : usize,
    radius: f64, // It is needed as f64 to be able to change the size
    mode: Mode,
    preview: Preview,
    current_color: Colors,
    background_color: Colors,
    window_size: (i32, i32),
    cursor_pos: (i32, i32),
    is_clicked: bool,
}

impl Canvas {
    pub fn new(surface: Surface<Rc<Window>, Rc<Window>>, window_size: (i32, i32)) -> Canvas {
        Canvas {
            drawing: vec![0; (window_size.0 * window_size.1) as usize],
            surface: surface,
            history: Vec::with_capacity(40),
            max_history: 40,
            radius: 2.0,
            mode: Mode::Drawing,
            preview: Preview::None,
            current_color: Colors::RED,
            background_color: Colors::TRANSPARENT,
            window_size,
            cursor_pos: (0, 0),
            is_clicked: false,
        }
    }
}
