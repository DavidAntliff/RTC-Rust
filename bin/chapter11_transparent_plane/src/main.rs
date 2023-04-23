// Chapter 11 Closing Challenge:
//
// Implement non-shadow-casting materials, so that a view from air into water
// is possible, without the water surface casting a shadow over the objects below.

use rust_rtc::camera::{camera, render};
use rust_rtc::canvas::ppm_from_canvas;
use rust_rtc::colors::{color, Color, colori};
use rust_rtc::lights::point_light;
use rust_rtc::materials::{default_material, RefractiveIndex};
use rust_rtc::math::MAX_RECURSIVE_DEPTH;
use rust_rtc::patterns::{checkers_pattern};
use rust_rtc::shapes::{plane, sphere};
use rust_rtc::transformations::{rotation_x, scaling, translation, view_transform};
use rust_rtc::tuples::{point, Point, vector};
use rust_rtc::world::world;
use std::f64::consts::PI;
use rand::prelude::*;
use rand_xoshiro::Xoshiro256StarStar;

fn main() {
    let mut w = world();

    let mut wall = plane();
    wall.set_transform(&rotation_x(PI / 2.0).then(&translation(0.0, 0.5, 10.0)));
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

    // depth of the water
    let water_depth = 2.5;

    let mut floor = plane();
    floor.set_transform(&translation(0.0, -water_depth, 0.0));
    floor.material = default_material();
    floor.material.color = colori(77, 63, 38);
    floor.material.ambient = 0.1;
    floor.material.diffuse = 0.9;
    floor.material.specular = 0.0;
    floor.material.reflective = 0.0;
    w.add_object(floor);

    struct Pebble {
        position: Point,
        size: f64,
        color: Color,
    }

    let mut rng = Xoshiro256StarStar::seed_from_u64(0);

    let num_pebbles = 500;
    let spread = 25.0;

    // Pebbles
    for i in 0..num_pebbles {
        let grey_level = { let v = rng.gen(); Color::new(v, v, v) };
        let size = 0.1 + rng.gen::<f64>() / 10.0;
        let pebble = Pebble {
            position: point(spread * (rng.gen::<f64>() - 0.5),
                            -size * rng.gen::<f64>(),
                            spread * (rng.gen::<f64>() - 0.5)),
            size,
            color: grey_level + Color::new(rng.gen::<f64>() / 10.0,
                                           rng.gen::<f64>() / 10.0,
                                           rng.gen::<f64>() / 10.0)
        };

        let mut object = sphere(i);
        object.set_transform(&scaling(pebble.size, pebble.size, pebble.size)
            .then(&translation(pebble.position.x(),
                               -water_depth + pebble.position.y(),
                               pebble.position.z())));
        object.material.color = pebble.color;
        object.material.specular = 0.2;
        object.material.shininess = 10.0;
        object.material.reflective = 0.0;
        w.add_object(object);
    }

    // Water plane - casts no shadow
    let mut water = plane();
    water.material = default_material();
    water.material.color = color(0.0, 0.1, 0.2);
    water.material.ambient = 0.2;
    water.material.diffuse = 0.6;
    water.material.specular = 0.0;
    water.material.shininess = 100.0;
    water.material.reflective = 0.7;
    water.material.transparency = 1.0;
    water.material.refractive_index = RefractiveIndex::WATER;
    water.material.casts_shadow = false;
    w.add_object(water);

    w.add_light(point_light(point(5.0, 10.0, -8.0), color(0.9, 0.9, 0.9)));

    //let mut cam = camera(100, 50, PI / 3.0);
    //let mut cam = camera(1024, 768, PI / 3.0);
    //let mut cam = camera(1600, 800, PI / 3.0);
    let mut cam = camera(2048, 1536, PI / 3.0);
    //let mut cam = camera(3840, 2160, PI / 3.0);

    // From the front
    cam.set_transform(
        &view_transform(
            &point(4.0, 4.0, -8.0),
            &point(0.0, 0.0, 0.0),
            &vector(0.0, 1.0, 0.0),
        )
        .then(&translation(0.0, 0.0, 2.0)),
    );

    let canvas = render(&cam, &w, MAX_RECURSIVE_DEPTH);
    let ppm = ppm_from_canvas(&canvas);
    print!("{}", ppm);
}
