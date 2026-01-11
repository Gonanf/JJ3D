mod core;
use std::time::Duration;
use sdl2::{event::Event, keyboard::Keycode, libc::sleep, pixels::Color, rect::Point,render::Canvas,video::Window};

fn main() {
    println!("Hello, world!");

    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();

    let window = video.window("amoga", 200, 200).build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.clear();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    core::shapes::CreateShape(&mut canvas,&[Point::new(100,0),Point::new(50,150),Point::new(150,150)]);
    core::shapes::CreateShape(&mut canvas,&[Point::new(50,0),Point::new(50,150),Point::new(150,150),Point::new(150,0)]);
    canvas.present();
    let mut x1 = 0;
    let mut x2 = 200;
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


       }
}
