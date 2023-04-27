use rust_rtc::colors::{color, BLACK, WHITE};
use rust_rtc::lights::point_light;

use rust_rtc::patterns::stripe_pattern;
use rust_rtc::shapes::{plane, sphere};
use rust_rtc::transformations::{scaling, translation, view_transform};
use rust_rtc::tuples::{point, vector};
use rust_rtc::utils;
use rust_rtc::utils::RenderOptions;
use rust_rtc::world::world;

use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = utils::parse_args();

    let mut w = world();

    let mut floor = plane();
    let floor_pattern = stripe_pattern(&BLACK, &WHITE);
    floor.material.set_pattern(&floor_pattern);
    w.add_object(floor);

    // No rotation of object, scaling or rotation or translation of patterns
    // should result in each sphere being exactly half black, half white.

    let mut middle = sphere(4);
    middle.set_transform(&translation(-0.5, 1.0, 0.5));
    let middle_pattern = stripe_pattern(&BLACK, &WHITE);
    middle.material.set_pattern(&middle_pattern);
    w.add_object(middle);

    let mut right = sphere(5);
    right.set_transform(&scaling(0.5, 0.5, 0.5).then(&translation(1.5, 0.5, -0.5)));
    let right_pattern = stripe_pattern(&BLACK, &WHITE);
    right.material.set_pattern(&right_pattern);
    w.add_object(right);

    let mut left = sphere(6);
    left.set_transform(&scaling(0.33, 0.33, 0.33).then(&translation(-1.5, 0.33, -0.75)));
    let left_pattern = stripe_pattern(&BLACK, &WHITE);
    left.material.set_pattern(&left_pattern);
    w.add_object(left);

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
