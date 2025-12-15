use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode, libc::sleep, pixels::Color, rect::Point};

fn main() {
    println!("Hello, world!");

    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();

    let window = video.window("amoga", 200, 200).build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.clear();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
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

        if y < 200 {
            for x in 0..200 {
                canvas.draw_point(Point::new(x, y));
            }
            y += 1;
            println!("Y : {y}, X : {x}");
            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
