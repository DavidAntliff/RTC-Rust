use rust_rtc::camera::{Resolution};

use rust_rtc::colors::{color, GREEN, GREY25, GREY75, RED};
use rust_rtc::lights::point_light;
use rust_rtc::materials::default_material;
use rust_rtc::math::MAX_RECURSIVE_DEPTH;
use rust_rtc::patterns::{blended_pattern, stripe_pattern};
use rust_rtc::shapes::{plane, sphere};
use rust_rtc::transformations::{rotation_y, scaling, translation, view_transform};
use rust_rtc::tuples::{point, vector};
use rust_rtc::world::world;
use std::f64::consts::PI;

fn main() {
    let mut w = world();

    let mut floor = plane();
    floor.material = default_material();
    floor.material.color = color(1.0, 0.9, 0.9);
    floor.material.ambient = 0.2;
    floor.material.specular = 0.0;
    floor.material.reflective = 0.2;
    let mut floor_pattern_1 = stripe_pattern(&GREY25, &GREY75);
    let mut floor_pattern_2 = stripe_pattern(&GREY25, &GREY75);
    let scale = 0.4;
    floor_pattern_1.set_transform(&scaling(scale, scale, scale).then(&rotation_y(PI / 4.0)));
    floor_pattern_2.set_transform(&scaling(scale, scale, scale).then(&rotation_y(-PI / 4.0)));
    let mut floor_pattern = blended_pattern(&floor_pattern_1, &floor_pattern_2);
    floor_pattern.set_transform(&rotation_y(-PI / 16.0));
    floor.material.set_pattern(&floor_pattern);
    w.add_object(floor);

    let mut sphere1 = sphere(1);
    sphere1.set_transform(&scaling(1.5, 1.5, 1.5).then(&translation(0.0, 0.5, 0.0)));
    sphere1.material.color = GREY25;
    sphere1.material.specular = 1.0;
    sphere1.material.shininess = 1000.0;
    sphere1.material.reflective = 0.9;
    w.add_object(sphere1);

    let mut sphere2 = sphere(2);
    sphere2.set_transform(&scaling(0.4, 0.4, 0.4).then(&translation(-2.0, 1.7, -1.0)));
    sphere2.material.color = RED;
    sphere2.material.specular = 0.7;
    sphere2.material.shininess = 100.0;
    sphere2.material.reflective = 0.4;
    w.add_object(sphere2);

    let mut sphere3 = sphere(3);
    sphere3.set_transform(&scaling(0.5, 0.5, 0.5).then(&translation(1.8, 1.8, -1.0)));
    sphere3.material.color = GREEN;
    sphere3.material.specular = 0.7;
    sphere3.material.shininess = 100.0;
    sphere3.material.reflective = 0.4;
    w.add_object(sphere3);

    let mut sphere4 = sphere(4);
    sphere4.set_transform(&scaling(0.2, 0.2, 0.2).then(&translation(1.5, 0.2, -1.8)));
    sphere4.material.color = color(0.8, 0.0, 0.8);
    sphere4.material.specular = 0.5;
    sphere4.material.shininess = 10.0;
    sphere4.material.reflective = 0.0;
    w.add_object(sphere4);

    let mut sphere5 = sphere(5);
    sphere5.set_transform(&scaling(0.5, 0.5, 0.5).then(&translation(-1.0, 0.5, -3.5)));
    sphere5.material.color = color(0.1, 0.0, 1.0);
    sphere5.material.specular = 1.0;
    sphere5.material.shininess = 1000.0;
    sphere5.material.reflective = 0.8;
    w.add_object(sphere5);

    // yellow sphere behind the camera
    let mut sphere6 = sphere(6);
    sphere6.set_transform(&scaling(2.0, 2.0, 2.0).then(&translation(4.0, 2.0, -5.0)));
    sphere6.material.color = color(0.9, 0.7, 0.0);
    sphere6.material.specular = 0.2;
    sphere6.material.shininess = 10.0;
    sphere6.material.reflective = 0.8;
    w.add_object(sphere6);

    w.add_light(point_light(point(-10.0, 10.0, -5.0), color(1.0, 1.0, 1.0)));

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
