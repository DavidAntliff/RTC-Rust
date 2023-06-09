use rust_rtc::colors::{color, BLACK, BLUE, GREEN, RED, WHITE, YELLOW};
use rust_rtc::lights::point_light;
use rust_rtc::materials::default_material;

use rust_rtc::patterns::checkers_pattern;
use rust_rtc::shapes::{plane, sphere};
use rust_rtc::transformations::{
    rotation_x, rotation_y, rotation_z, scaling, translation, view_transform,
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
    floor.material = default_material();
    floor.material.color = color(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;
    let mut floor_pattern = checkers_pattern(&RED, &WHITE);
    floor_pattern.set_transform(&rotation_y(PI / 4.0));
    floor.material.set_pattern(&floor_pattern);
    w.add_object(floor);

    let mut wall = plane();
    wall.set_transform(
        &rotation_x(PI / 2.0)
            .then(&rotation_y(0.3))
            .then(&translation(0.0, 0.0, 7.0)),
    );
    wall.material = default_material();
    wall.material.color = color(1.0, 0.8, 0.8);
    wall.material.diffuse = 0.3;
    wall.material.specular = 0.0;
    wall.material.set_pattern(&checkers_pattern(&BLACK, &RED));
    w.add_object(wall);

    let mut middle = sphere(4);
    middle.set_transform(&translation(-0.5, 1.0, 0.5));
    middle.material = default_material();
    middle.material.color = color(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    let mut middle_pattern = checkers_pattern(&BLUE, &WHITE);
    middle_pattern.set_transform(&scaling(0.2, 0.2, 0.2).then(&rotation_y(-PI / 8.0)));
    middle.material.set_pattern(&middle_pattern);
    w.add_object(middle);

    let mut right = sphere(5);
    right.set_transform(&scaling(0.5, 0.5, 0.5).then(&translation(1.5, 0.5, -0.5)));
    right.material = default_material();
    right.material.color = color(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    let mut right_pattern = checkers_pattern(&GREEN, &BLACK);
    right_pattern.set_transform(&scaling(0.1, 0.1, 0.1).then(&rotation_z(-PI / 6.0)));
    right.material.set_pattern(&right_pattern);
    w.add_object(right);

    let mut left = sphere(6);
    left.set_transform(
        &rotation_z(PI / 4.0)
            .then(&scaling(0.33, 0.33, 0.33).then(&translation(-1.5, 0.33, -0.75))),
    );
    left.material = default_material();
    left.material.color = color(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    let mut left_pattern = checkers_pattern(&YELLOW, &BLACK);
    left_pattern.set_transform(&scaling(0.15, 0.15, 0.15));
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
