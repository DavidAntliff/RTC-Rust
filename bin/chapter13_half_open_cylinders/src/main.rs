use rust_rtc::colors::color;
use rust_rtc::lights::point_light;

use rust_rtc::shapes::cylinder;
use rust_rtc::transformations::{rotation_x, rotation_y, scaling, translation, view_transform};
use rust_rtc::tuples::{point, vector};
use rust_rtc::utils;
use rust_rtc::utils::RenderOptions;
use rust_rtc::world::world;
use std::f64::consts::PI;
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = utils::parse_args();

    let mut w = world();

    let cyl_len = 4.0;
    let solid = false;

    // Steel pipes
    let mut cyl = cylinder(-cyl_len * 1.0, cyl_len * 1.0, true, solid);
    cyl.set_transform(
        &rotation_x(-PI / 2.0)
            .then(&rotation_x(0.1))
            .then(&rotation_y(0.2))
            .then(&translation(4.0, 4.0, -6.0)),
    );
    cyl.material.color = color(0.7922, 0.80, 0.8078);
    cyl.material.ambient = 0.2;
    cyl.material.diffuse = 0.3;
    cyl.material.specular = 0.8;
    cyl.material.shininess = 100.0;
    cyl.material.shininess = 10.0;
    cyl.material.reflective = 0.5;
    w.add_object(cyl);

    let mut cyl = cylinder(-cyl_len * 5.0, cyl_len * 1.0, true, solid);
    cyl.set_transform(
        &rotation_x(-PI / 2.0)
            .then(&rotation_y(-0.33))
            .then(&rotation_x(0.3))
            .then(&translation(-6.0, -1.0, -4.0)),
    );
    cyl.material.color = color(0.7922, 0.80, 0.8078);
    cyl.material.ambient = 0.2;
    cyl.material.diffuse = 0.3;
    cyl.material.specular = 0.8;
    cyl.material.shininess = 100.0;
    cyl.material.shininess = 10.0;
    cyl.material.reflective = 0.5;
    w.add_object(cyl);

    // Copper pipes
    let mut cyl = cylinder(-cyl_len * 1.0, cyl_len * 1.0, true, solid);
    cyl.set_transform(&rotation_x(-PI / 2.0).then(&translation(-3.0, 4.0, -4.0)));
    cyl.material.color = color(0.722, 0.451, 0.20);
    cyl.material.ambient = 0.2;
    cyl.material.diffuse = 0.3;
    cyl.material.specular = 0.8;
    cyl.material.shininess = 100.0;
    cyl.material.shininess = 10.0;
    cyl.material.reflective = 0.5;
    w.add_object(cyl);

    let mut cyl = cylinder(-cyl_len * 1.0, cyl_len * 1.0, true, solid);
    cyl.set_transform(&rotation_x(-PI / 2.0).then(&translation(0.0, 2.0, -4.0)));
    cyl.material.color = color(0.722, 0.451, 0.20);
    cyl.material.ambient = 0.2;
    cyl.material.diffuse = 0.3;
    cyl.material.specular = 0.8;
    cyl.material.shininess = 100.0;
    cyl.material.shininess = 10.0;
    cyl.material.reflective = 0.5;
    w.add_object(cyl);

    let mut cyl = cylinder(-cyl_len * 1.5, cyl_len * 1.0, true, solid);
    cyl.set_transform(
        &rotation_x(-PI / 2.0)
            .then(&rotation_x(0.4))
            .then(&translation(4.5, -2.0, -4.0)),
    );
    cyl.material.color = color(0.722, 0.451, 0.20);
    cyl.material.ambient = 0.2;
    cyl.material.diffuse = 0.3;
    cyl.material.specular = 0.8;
    cyl.material.shininess = 100.0;
    cyl.material.shininess = 10.0;
    cyl.material.reflective = 0.5;
    w.add_object(cyl);

    let mut cyl = cylinder(-cyl_len * 1.5, cyl_len * 1.0, true, solid);
    cyl.set_transform(
        &scaling(3.0, 0.1, 3.0)
            .then(&rotation_x(-0.1))
            .then(&translation(-1.0, -3.0, -4.0)),
    );
    cyl.material.color = color(0.722, 0.451, 0.20);
    cyl.material.ambient = 0.2;
    cyl.material.diffuse = 0.3;
    cyl.material.specular = 0.8;
    cyl.material.shininess = 100.0;
    cyl.material.shininess = 10.0;
    cyl.material.reflective = 0.5;
    w.add_object(cyl);

    w.add_light(point_light(point(-2.0, 5.0, -10.0), color(1.0, 1.0, 1.0)));

    let options = RenderOptions {
        camera_transform: view_transform(
            &point(0.0, 1.5, -5.0),
            &point(0.0, 0.5, 0.0),
            &vector(0.0, 1.0, 0.0),
        )
        .then(&translation(0.0, 0.0, -20.0)),
        ..Default::default()
    };

    ExitCode::from(match utils::render_world(&w, options, &cli) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("Write {}: {}", cli.output, e);
            1
        }
    })
}
