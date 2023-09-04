use rust_rtc::colors::{color, colori, RED, WHITE};
use rust_rtc::lights::point_light;

use rust_rtc::materials::default_material;
use rust_rtc::patterns::{checkers_pattern, stripe_pattern};
use rust_rtc::shapes::{cone, plane};
use rust_rtc::transformations::{
    rotation_x, rotation_y, rotation_z, scaling, translate_y, translation, view_transform,
};
use rust_rtc::tuples::{point, vector};
use rust_rtc::utils;
use rust_rtc::utils::RenderOptions;
use rust_rtc::world::world;
use std::f64::consts::PI;
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = utils::parse_args();

    let mut w = world();

    let mut floor = plane();
    floor.set_transform(&translate_y(-1.0));
    floor.material = default_material();
    floor.material.color = color(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;
    floor.material.reflective = 0.2;
    let scale = 0.3;
    let mut floor_pattern_1 = stripe_pattern(&colori(167, 83, 104), &colori(124, 41, 62));
    floor_pattern_1.set_transform(&scaling(scale, scale, scale).then(&rotation_y(PI / 4.0)));
    let mut floor_pattern_2 = stripe_pattern(&colori(63, 63, 63), &colori(104, 104, 104));
    floor_pattern_2.set_transform(&scaling(scale, scale, scale).then(&rotation_y(-PI / 4.0)));
    let mut floor_pattern = checkers_pattern(&floor_pattern_1, &floor_pattern_2);
    floor_pattern.set_transform(&scaling(1.0, 1.0, 1.0));
    floor.material.set_pattern(&floor_pattern);
    w.add_object(floor);

    let mut c = cone();
    c.set_transform(&translation(-10.0, 0.0, 0.0));
    c.material.color = color(1.0, 0.843, 0.0);
    c.material.ambient = 0.2;
    c.material.specular = 1.0;
    c.material.shininess = 1000.0;
    c.material.reflective = 1.0;
    w.add_object(c);

    let mut c = cone();
    let p = c.as_cone_primitive().expect("should be a cone");
    p.maximum_y = 1.0;
    c.set_transform(&translation(-5.0, 0.0, -2.0));
    c.material.color = color(0.2, 0.0, 0.9);
    c.material.specular = 1.0;
    c.material.shininess = 1000.0;
    c.material.reflective = 1.0;
    w.add_object(c);

    let mut c = cone();
    let p = c.as_cone_primitive().expect("should be a cone");
    p.maximum_y = 2.3;
    c.set_transform(&translation(0.0, 0.0, 0.0));
    c.material.color = RED;
    c.material.reflective = 1.0;
    w.add_object(c);

    let mut c = cone();
    let p = c.as_cone_primitive().expect("should be a cone");
    p.minimum_y = -2.0;
    c.set_transform(&rotation_x(-PI / 4.0).then(&translation(6.0, 1.9, 1.0)));
    c.material.color = color(0.0, 0.9, 0.1);
    c.material.specular = 1.0;
    c.material.shininess = 300.0;
    c.material.reflective = 1.0;
    w.add_object(c);

    let mut c = cone();
    let p = c.as_cone_primitive().expect("should be a cone");
    p.minimum_y = -2.0;
    c.set_transform(
        &rotation_z(-PI / 4.0)
            .then(&rotation_y(-0.3))
            .then(&translation(12.0, 1.9, 0.0)),
    );
    c.material.color = WHITE;
    c.material.ambient = 0.2;
    c.material.diffuse = 1.0;
    c.material.specular = 1.0;
    c.material.shininess = 300.0;
    c.material.reflective = 0.9;
    w.add_object(c);

    w.add_light(point_light(point(-2.0, 5.0, -10.0), color(1.0, 1.0, 1.0) / 2.0));
    w.add_light(point_light(point(5.0, 5.0, -10.0), color(1.0, 1.0, 1.0) / 2.0));
    w.add_light(point_light(point(0.0, 25.0, 100.0), color(0.7, 0.0, 0.0)));

    let options = RenderOptions {
        camera_transform: view_transform(
            &point(0.0, 1.5, -5.0),
            &point(0.0, 0.5, 0.0),
            &vector(0.0, 1.0, 0.0),
        )
        .then(&translation(0.0, 0.0, -20.0)),
        ..Default::default()
    };

    ExitCode::from(match utils::render_world(&w, options, &cli.common) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("Write {}: {}", cli.common.render.output, e);
            1
        }
    })
}
