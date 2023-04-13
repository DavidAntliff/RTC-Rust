use std::f64::consts::PI;

use rust_rtc::canvas::{canvas, ppm_from_canvas, write_pixel, Canvas};
use rust_rtc::colors::color;
use rust_rtc::transformations::rotation_z;
use rust_rtc::tuples::point;

fn set_pixel(c: &mut Canvas, x: f64, y: f64) {
    let x_off = f64::from(c.width / 2);
    let y_off = f64::from(c.height / 2);
    let scaling = 1.0;

    let x = (scaling * x + x_off) as u32;
    let y = (scaling * y + y_off) as u32;

    //eprintln!("write_pixel: {}, {}", x, y);
    write_pixel(c, x, y, &color(1.0, 1.0, 1.0));
}

fn main() {
    let mut c = canvas(900, 550);

    // 1 hour rotation (15 degrees)
    let hour = rotation_z(2.0 * PI / 12.0);

    set_pixel(&mut c, 0.0, 0.0);

    let mut p = point(0.0, 120.0, 0.0);

    for _i in 0..12 {
        eprintln!("{:?}", p);
        let p2 = &hour * p;
        set_pixel(&mut c, p2.x(), p2.y());
        p = p2;
    }

    let ppm = ppm_from_canvas(&c);
    println!("{}", ppm);
}
