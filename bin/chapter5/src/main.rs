use rust_rtc::canvas::{canvas, ppm_from_canvas, write_pixel};
use rust_rtc::colors::RED;
use rust_rtc::intersections::{hit, intersect};
use rust_rtc::rays::ray;
use rust_rtc::shapes::sphere;
use rust_rtc::transformations::{scaling, /*rotation_z,*/ shearing};
use rust_rtc::tuples::{normalize, point};

fn main() {
    // Sphere is at 0.0, 0.0, 0.0
    // Ray origin is z = -5.0
    // Wall is z = 10.0

    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z = 10.0;

    // Good size for wall is > 6 units for entire shadow
    let wall_size = 7.0;

    let canvas_pixels = 100;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut c = canvas(canvas_pixels, canvas_pixels);

    let red = RED;
    let mut shape = sphere(1);

    // shrink along the y axis
    //shape.set_transform(scaling(1.0, 0.5, 1.0));

    // shrink along the x axis
    //shape.set_transform(scaling(0.5, 1.0, 1.0));

    // shrink it and rotate it
    //shape.set_transform(rotation_z(pi / 4) * scaling(0.5, 1.0, 1.0));

    // shrink it and skew it
    shape.set_transform(&(shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * scaling(0.5, 1.0, 1.0)));

    // for each row of pixels in the canvas
    for y in 0..canvas_pixels {
        // compute the world y coordinate (top = +half, bottom = -half)
        let world_y = half - pixel_size * y as f64;
        // for each pixel in the row
        for x in 0..canvas_pixels {
            // compute the world x coordinate (left = -half, right = +half)
            let world_x = -half + pixel_size * x as f64;
            // describe the point on the wall that the ray will target
            let position = point(world_x, world_y, wall_z);

            let r = ray(ray_origin, normalize(&(position - ray_origin)));
            let mut xs = intersect(&shape, &r);

            if hit(&mut xs).is_some() {
                write_pixel(&mut c, x, y, &red);
            }
        }
    }

    let ppm = ppm_from_canvas(&c);
    println!("{}", ppm);
}
