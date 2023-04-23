// Scene from book author: https://forum.raytracerchallenge.com/post/438/thread
//
// - add: camera
//   width: 300
//   height: 300
//   field-of-view: 0.45
//   from: [ 0, 0, -5 ]
//   to: [ 0, 0, 0 ]
//   up: [ 0, 1, 0 ]
//
// - add: light
//   intensity: [ 0.9, 0.9, 0.9 ]
//   at: [ 2, 10, -5 ]
//
// # wall
// - add: plane
//   transform:
//     - [ rotate-x, 1.5708 ]
//     - [ translate, 0, 0, 10 ]
//   material:
//     pattern:
//       type: checkers
//       colors:
//         - [ 0.15, 0.15, 0.15 ]
//         - [ 0.85, 0.85, 0.85 ]
//     ambient: 0.8
//     diffuse: 0.2
//     specular: 0
//
// # glass ball
// - add: sphere
//   material:
//     color: [ 1, 1, 1 ]
//     ambient: 0
//     diffuse: 0
//     specular: 0.9
//     shininess: 300
//     reflective: 0.9
//     transparency: 0.9
//     refractive-index: 1.5
//
// # hollow center
// - add: sphere
//   transform:
//     - [ scale, 0.5, 0.5, 0.5 ]
//   material:
//     color: [ 1, 1, 1 ]
//     ambient: 0
//     diffuse: 0
//     specular: 0.9
//     shininess: 300
//     reflective: 0.9
//     transparency: 0.9
//     refractive-index: 1.0000034

use rust_rtc::camera::{Resolution};

use rust_rtc::colors::{color, WHITE};
use rust_rtc::lights::point_light;
use rust_rtc::materials::default_material;
use rust_rtc::math::MAX_RECURSIVE_DEPTH;
use rust_rtc::patterns::checkers_pattern;
use rust_rtc::shapes::{plane, sphere};
use rust_rtc::transformations::{rotation_x, scaling, translation, view_transform};
use rust_rtc::tuples::{point, vector};
use rust_rtc::world::world;
use std::f64::consts::PI;

fn main() {
    let mut w = world();

    let mut wall = plane();
    wall.set_transform(&rotation_x(PI / 2.0).then(&translation(0.0, 0.0, 10.0)));
    wall.material = default_material();
    wall.material.color = color(1.0, 0.9, 0.9);
    wall.material.ambient = 0.8;
    wall.material.diffuse = 0.2;
    wall.material.specular = 0.0;
    wall.material.reflective = 0.2;
    wall.material.set_pattern(&checkers_pattern(
        &color(0.15, 0.15, 0.15),
        &color(0.85, 0.85, 0.85),
    ));
    w.add_object(wall);

    // Glass ball
    let mut sphere1 = sphere(1);
    sphere1.material.color = WHITE;
    sphere1.material.ambient = 0.0;
    sphere1.material.diffuse = 0.0;
    sphere1.material.specular = 0.9;
    sphere1.material.shininess = 300.0;
    sphere1.material.reflective = 0.9;
    sphere1.material.transparency = 0.9;
    sphere1.material.refractive_index = 1.5;
    w.add_object(sphere1);

    // Hollow centre
    let mut sphere2 = sphere(2);
    sphere2.set_transform(&scaling(0.5, 0.5, 0.5));
    sphere2.material.color = WHITE;
    sphere2.material.ambient = 0.0;
    sphere2.material.diffuse = 0.0;
    sphere2.material.specular = 0.9;
    sphere2.material.shininess = 300.0;
    sphere2.material.reflective = 0.9;
    sphere2.material.transparency = 0.9;
    sphere2.material.refractive_index = 1.0000034;
    w.add_object(sphere2);

    w.add_light(point_light(point(2.0, 10.0, -5.0), color(0.9, 0.9, 0.9)));

    //let resolution = Resolution::VGA;  // 640 x 480
    //let resolution = Resolution::XGA;  // 1024 x 768
    //let resolution = Resolution::QHD;  // 2560 x 1440
    //let resolution = Resolution::UHD_4K;  // 3840 x 2160
    let resolution = Resolution::new(600, 600);

    let camera_transform = view_transform(
        &point(0.0, 0.0, -5.0),
        &point(0.0, 0.0, 0.0),
        &vector(0.0, 1.0, 0.0),
    );

    rust_rtc::utils::render_world(&w, resolution, 0.45, camera_transform, MAX_RECURSIVE_DEPTH);
}
