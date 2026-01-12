use sdl2::{event::Event, keyboard::Keycode, libc::sleep, pixels::Color, rect::Point, render::Canvas, video::Window};
use super::lines::CreateLine;
use super::objects::ShapeObject;
use super::prelude::*;

//TODO: Fabric?

pub struct Circle<'a> {
    object: ShapeObject<'a>,
    radio: i32
}

impl<'a> Shape for Circle<'a> {
    fn draw(&mut self){
        let mut x = 0;
        let mut y = -self.radio;
        let mut p = -self.radio;

        while x < -y{
            if p > 0 {
                y += 1;
                p += 2*(x+y)+1;
            }else{
                p += 2*x +1;
            }

            
            self.object.canvas.draw_point(Point::new(self.object.position.x + x,self.object.position.y + y));
            self.object.canvas.draw_point(Point::new(self.object.position.x - x,self.object.position.y + y));
            self.object.canvas.draw_point(Point::new(self.object.position.x + x,self.object.position.y - y));
            self.object.canvas.draw_point(Point::new(self.object.position.x - x,self.object.position.y - y));
            self.object.canvas.draw_point(Point::new(self.object.position.x + y,self.object.position.y + x));
            self.object.canvas.draw_point(Point::new(self.object.position.x + y,self.object.position.y - x));
            self.object.canvas.draw_point(Point::new(self.object.position.x - y,self.object.position.y + x));
            self.object.canvas.draw_point(Point::new(self.object.position.x - y,self.object.position.y - x));
            x+=1;
        }
    }
}

impl<'a> Circle<'a> {
    pub fn new(canvas: &mut Canvas<Window>, radio: i32, position: Option<Point>) -> Circle { 
        return Circle { object: 
            ShapeObject {
                canvas: canvas,
                position: position.unwrap_or(Point::new(0,0)),
                transform: Point::new(0,0),
                rotation: 0
            },
            radio: radio
        }
    }
}
