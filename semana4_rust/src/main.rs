mod core;
use std::time::Duration;
use sdl2::{event::Event, keyboard::Keycode, libc::sleep, pixels::Color, rect::Point,render::Canvas,video::Window};
use core::shapes::prelude::*;

fn main() {
    println!("Hello, world!");

    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();

    let window = video.window("amoga", 200, 200).build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.clear();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    core::shapes::rects::Rect::new(&mut canvas,[Point::new(100,0),Point::new(50,150),Point::new(150,150)]).draw();
    core::shapes::rects::Rect::new(&mut canvas,[Point::new(50,0),Point::new(50,150),Point::new(150,150),Point::new(150,0)]).draw();
    let mut x = 0;
    let mut y = 0;
    let mut event_pump = context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }


    if x >= 200 { x = 0; }
    if y >= 200 { y = 0; }
    core::shapes::circles::Circle::new(&mut canvas, 16, Some(Point::new(x,y))).draw();
    canvas.present();
    x+=1;
    y+=1;
    
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
       }
}
