use rust_rtc::colors::{color, colori};
use rust_rtc::lights::point_light;
use rust_rtc::materials::default_material;

use rust_rtc::patterns::{checkers_pattern, stripe_pattern};
use rust_rtc::shapes::plane;
use rust_rtc::transformations::{rotation_y, scaling, view_transform};
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
    floor.material = default_material();
    floor.material.color = color(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;
    let scale = 0.3;
    let mut floor_pattern_1 = stripe_pattern(&colori(167, 83, 104), &colori(124, 41, 62));
    floor_pattern_1.set_transform(&scaling(scale, scale, scale).then(&rotation_y(PI / 4.0)));
    let mut floor_pattern_2 = stripe_pattern(&colori(63, 63, 63), &colori(104, 104, 104));
    floor_pattern_2.set_transform(&scaling(scale, scale, scale).then(&rotation_y(-PI / 4.0)));
    let mut floor_pattern = checkers_pattern(&floor_pattern_1, &floor_pattern_2);
    floor_pattern.set_transform(&scaling(1.0, 1.0, 1.0));
    floor.material.set_pattern(&floor_pattern);
    w.add_object(floor);

    w.add_light(point_light(point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0)));

    let options = RenderOptions {
        camera_transform: view_transform(
            &point(0.0, 1.5, -5.0),
            &point(0.0, 1.0, 0.0),
            &vector(0.0, 1.0, 0.0),
        ),
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
