use rust_rtc::camera::{Resolution};

use rust_rtc::colors::color;
use rust_rtc::lights::point_light;
use rust_rtc::materials::default_material;
use rust_rtc::math::MAX_RECURSIVE_DEPTH;
use rust_rtc::shapes::sphere;
use rust_rtc::transformations::{rotation_x, rotation_y, scaling, translation, view_transform};
use rust_rtc::tuples::{point, vector};
use rust_rtc::world::world;
use std::f64::consts::PI;

fn main() {
    let mut w = world();
    let mut floor = sphere(1);
    floor.set_transform(&scaling(10.0, 0.01, 10.0));
    floor.material = default_material();
    floor.material.color = color(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;
    w.add_object(floor.clone());

    let mut left_wall = sphere(2);
    left_wall.set_transform(
        &(translation(0.0, 0.0, 5.0)
            * rotation_y(-PI / 4.0)
            * rotation_x(PI / 2.0)
            * scaling(10.0, 0.01, 10.0)),
    );
    left_wall.material = floor.material.clone();
    w.add_object(left_wall);

    let mut right_wall = sphere(3);
    right_wall.set_transform(
        &(translation(0.0, 0.0, 5.0)
            * rotation_y(PI / 4.0)
            * rotation_x(PI / 2.0)
            * scaling(10.0, 0.01, 10.0)),
    );
    right_wall.material = floor.material;
    w.add_object(right_wall);

    let mut middle = sphere(4);
    middle.set_transform(&translation(-0.5, 1.0, 0.5));
    middle.material = default_material();
    middle.material.color = color(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    w.add_object(middle);

    let mut right = sphere(5);
    right.set_transform(&(translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5)));
    right.material = default_material();
    right.material.color = color(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    w.add_object(right);

    let mut left = sphere(6);
    left.set_transform(&(translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33)));
    left.material = default_material();
    left.material.color = color(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    w.add_object(left);

    let mut left_up = sphere(7);
    left_up.set_transform(&(translation(-2.0, 1.8, -1.0) * scaling(0.33, 0.33, 0.33)));
    left_up.material = default_material();
    left_up.material.color = color(1.0, 0.0, 0.0);
    left_up.material.diffuse = 0.7;
    left_up.material.specular = 0.6;
    w.add_object(left_up);

    w.add_light(point_light(point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0)));

    //let resolution = Resolution::VGA;  // 640 x 480
    //let resolution = Resolution::XGA;  // 1024 x 768
    let resolution = Resolution::QHD; // 2560 x 1440
                                      //let resolution = Resolution::UHD_4K;  // 3840 x 2160

    let camera_transform = view_transform(
        &point(0.0, 1.5, -5.0),
        &point(0.0, 1.0, 0.0),
        &vector(0.0, 1.0, 0.0),
    );

    rust_rtc::utils::render_world(
        &w,
        resolution,
        PI / 3.0,
        camera_transform,
        MAX_RECURSIVE_DEPTH,
    );
}
