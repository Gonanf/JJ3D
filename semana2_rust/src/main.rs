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


        if (x1 >= 200) { x1 = 0 };
        if (x2 <= 0) { x2 = 200 };

        println!("X1: {}, X2: {}",x1,x2);
        core::lines::CreateLine(&mut canvas,Point::new(x1,0),Point::new(x2,200));
        canvas.present();
        x1+=1;
        x2-=1;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
