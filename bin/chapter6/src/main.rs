use std::f64::consts::PI;

use rust_rtc::canvas::{canvas, ppm_from_canvas, write_pixel};
use rust_rtc::colors::color;
use rust_rtc::intersections::{hit, intersect};
use rust_rtc::lights::point_light;
use rust_rtc::materials::{lighting, material};
use rust_rtc::rays::ray;
use rust_rtc::spheres::sphere;
use rust_rtc::transformations::{rotation_z, scaling};
use rust_rtc::tuples::{normalize, point};

fn main() {
    // Sphere is at 0.0, 0.0, 0.0
    // Ray origin is z = -5.0
    // Wall is z = 10.0

    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z = 10.0;

    // Good size for wall is > 6 units for entire shadow
    let wall_size = 7.0;

    let canvas_pixels = 200;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut c = canvas(canvas_pixels, canvas_pixels);
    //let red = color(1.0, 0.0, 0.0);
    let mut shape = sphere(1);

    let mut mat = material();
    //mat.set_color(color(1.0, 0.2, 1.0));
    mat.color = color(0.2, 1.0, 0.2);
    mat.specular = 0.5;
    mat.diffuse = 0.4;
    mat.shininess = 10.0;
    shape.material = mat;

    // shrink along the y axis
    //shape.set_transform(scaling(1.0, 0.5, 1.0));

    // shrink along the x axis
    //shape.set_transform(scaling(0.5, 1.0, 1.0));

    // shrink it and rotate it
    shape.set_transform(&(rotation_z(-PI / 3.0) * scaling(0.5, 1.0, 1.0)));

    // shrink it and skew it
    //shape.set_transform(shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * scaling(0.5, 1.0, 1.0));

    //let light_position = point(-10.0, 10.0, -10.0);
    let light_position = point(10.0, -10.0, -10.0);
    //let light_color = color(1.0, 1.0, 1.0);
    let light_color = color(1.0, 1.0, 1.0);
    let light = point_light(light_position, light_color);

    // for each row of pixels in the canvas
    for y in 0..canvas_pixels {
        if y % 50 == 0 {
            eprintln!("row {}", y);
        }

        // compute the world y coordinate (top = +half, bottom = -half)
        let world_y = half - pixel_size * y as f64;
        // for each pixel in the row
        for x in 0..canvas_pixels {
            // compute the world x coordinate (left = -half, right = +half)
            let world_x = -half + pixel_size * x as f64;
            // describe the point on the wall that the ray will target
            let pos = point(world_x, world_y, wall_z);

            let r = ray(ray_origin, normalize(&(pos - ray_origin)));
            let mut xs = intersect(&shape, &r);

            if let Some(h) = hit(&mut xs) {
                let point = r.position(h.t);
                let normal = h.object.local_normal_at(&point);
                let eye = -r.direction;

                let color = lighting(
                    &h.object.material,
                    h.object,
                    &light,
                    &point,
                    &eye,
                    &normal,
                    false,
                );

                write_pixel(&mut c, x, y, &color);
            }
        }
    }

    let ppm = ppm_from_canvas(&c);
    println!("{}", ppm);
}