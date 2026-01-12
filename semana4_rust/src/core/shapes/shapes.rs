use sdl2::{event::Event, keyboard::Keycode, libc::sleep, pixels::Color, rect::Point, render::Canvas, video::Window};
use crate::core::lines::CreateLine;


pub trait Shape {
    fn draw(&mut self);
}

pub struct ShapeObject<'a> {
    canvas: &'a mut Canvas<Window>,
    position: Point,
    transform: Point,
    rotation: i8
}

pub struct Rect<'a,const N: usize> {
    object: ShapeObject<'a>,
    points: [Point; N]
}

impl<'a,const N: usize> Shape for Rect<'a,N> {
    fn draw(&mut self){
        for p in 0..self.points.len()-1 {
            CreateLine(&mut self.object.canvas,self.points[p],self.points[p+1]);
        }
        CreateLine(&mut self.object.canvas,self.points[0],self.points[self.points.len() -1]);
    }
}

impl<'a,const N: usize> Rect<'a,N> {
    pub fn new(canvas: &mut Canvas<Window>, points: [Point; N]) -> Rect<{ N }> { 
        return Rect { object: 
            ShapeObject {
                canvas: canvas,
                position: Point::new(0,0),
                transform: Point::new(0,0),
                rotation: 0
            },
            points: points
        }
    }
}

pub struct Circle<'a> {
    object: ShapeObject<'a>,
    radio: i32
}
