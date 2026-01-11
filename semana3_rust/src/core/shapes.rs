use sdl2::{event::Event, keyboard::Keycode, libc::sleep, pixels::Color, rect::Point, render::Canvas, video::Window};
use crate::core::lines::CreateLine;


pub fn CreateShape(canvas: &mut Canvas<Window>,points: &[Point]){
    for p in 0..points.len()-1 {
        CreateLine(canvas,points[p],points[p+1]);
    }
    CreateLine(canvas,points[0],points[points.len() -1]);
}
