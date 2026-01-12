use sdl2::{event::Event, keyboard::Keycode, libc::sleep, pixels::Color, rect::Point, render::Canvas, video::Window};

//TODO: Make this into a structure helper or implementation into Canvas
pub fn CreateLine(canvas: &mut  Canvas<Window>,starting_point: Point, final_point: Point){
    if ((final_point.x-starting_point.x).abs() > (final_point.y-starting_point.y).abs()){
        _CreateLineX(canvas,starting_point,final_point);
    }
    else{
        _CreateLineY(canvas,starting_point,final_point);
    }
}

fn _CreateLineX(canvas: &mut Canvas<Window>, starting_point: Point, final_point: Point){
    let sp = if starting_point.x > final_point.x { final_point } else { starting_point };
    let fp = if starting_point.x > final_point.x { starting_point } else { final_point };
    let delta_x = fp.x - sp.x;
    let mut delta_y = fp.y - sp.y;

    let dir = if delta_y < 0 { -1 } else { 1 };
    delta_y *= dir;

    if delta_x != 0{
        let mut y = sp.y as i32;
        let mut p = 2*delta_y - delta_x;
        for i in 0..delta_x+1{
            // println!("X LINE: ({},{})",sp.x+i,y);
            canvas.draw_point(Point::new(sp.x+i,y));
            if p >= 0{
                y += dir;
                p = p - 2*delta_x;
            }
            p = p + 2*delta_y;
        }
}
}

fn _CreateLineY(canvas: &mut Canvas<Window>,starting_point: Point, final_point: Point){
    let sp = if starting_point.y > final_point.y { final_point } else { starting_point };
    let fp = if starting_point.y > final_point.y { starting_point } else { final_point };
    let mut delta_x = fp.x - sp.x;
    let delta_y = fp.y - sp.y;

    let dir = if delta_x < 0 { -1 } else { 1 };
    delta_x *= dir;

    if delta_y != 0{
        let mut x = sp.x;
        let mut p = 2*delta_x - delta_y;
        for i in 0..delta_y+1{
            // println!("Y LINE: ({},{})",x,sp.y+i);
            canvas.draw_point(Point::new(x,sp.y+i));
            if p >= 0{
                x += dir;
                p = p - 2*delta_y;
            }
            p = p + 2*delta_x;
        }
}
}
