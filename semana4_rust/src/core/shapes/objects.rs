use sdl2::{event::Event, keyboard::Keycode, libc::sleep, pixels::Color, rect::Point, render::Canvas, video::Window};


pub struct ShapeObject<'a> {
    pub canvas: &'a mut Canvas<Window>,
    pub position: Point,
    pub transform: Point,
    pub rotation: i8
}



