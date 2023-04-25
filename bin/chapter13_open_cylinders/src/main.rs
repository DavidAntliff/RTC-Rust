use rust_rtc::colors::color;
use rust_rtc::lights::point_light;

use rust_rtc::shapes::{cylinder, infinite_cylinder};
use rust_rtc::transformations::{
    rotation_x, rotation_z, translate_y, translate_z, translation, view_transform,
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

    let cyl_len = 4.0;

    //let mut cyl1 = cylinder(-cyl_len * 1.5, cyl_len, false, false);
    let mut cyl1 = infinite_cylinder();
    let cyl_prim = cyl1.as_cylinder_primitive().expect("should be a cylinder");
    cyl_prim.minimum_y = -cyl_len * 1.5;
    cyl_prim.maximum_y = cyl_len * 0.5;
    cyl_prim.closed_min = false;
    cyl_prim.closed_max = false;
    cyl1.material.color = color(0.722, 0.451, 0.20);
    cyl1.material.specular = 1.0;
    cyl1.material.shininess = 10.0;
    cyl1.material.shininess = 10.0;
    cyl1.material.reflective = 0.9;
    w.add_object(cyl1);

    let mut cyl2 = cylinder(-cyl_len * 2.0, cyl_len * 4.0, false, false);
    cyl2.set_transform(
        &rotation_z(PI / 4.0)
            .then(&rotation_x(PI / 2.0))
            .then(&translate_z(-5.0)),
    );
    cyl2.material.color = color(0.722, 0.451, 0.20);
    cyl2.material.specular = 1.0;
    cyl2.material.shininess = 10.0;
    cyl2.material.shininess = 10.0;
    cyl2.material.reflective = 0.9;
    w.add_object(cyl2);

    let mut cyl3 = cylinder(-cyl_len * 2.0, cyl_len * 3.0, false, false);
    cyl3.set_transform(
        &rotation_z(-PI / 4.0)
            .then(&rotation_x(PI / 2.0))
            .then(&translate_y(4.0))
            .then(&translate_z(5.0)),
    );
    cyl3.material.color = color(0.722, 0.451, 0.20);
    cyl3.material.specular = 1.0;
    cyl3.material.shininess = 10.0;
    cyl3.material.shininess = 10.0;
    cyl3.material.reflective = 0.9;
    w.add_object(cyl3);

    let mut cyl4 = cylinder(-cyl_len * 3.0, -cyl_len * 1.5, false, false);
    cyl4.set_transform(&translation(-10.5, 0.0, 10.0));
    cyl4.material.color = color(0.7922, 0.80, 0.8078);
    cyl4.material.diffuse = 0.3;
    cyl4.material.specular = 0.8;
    cyl4.material.shininess = 100.0;
    cyl4.material.shininess = 10.0;
    cyl4.material.reflective = 0.5;
    w.add_object(cyl4);

    let mut cyl5 = cylinder(-cyl_len * 1.0, cyl_len * 1.0, false, false);
    cyl5.set_transform(&rotation_x(-PI / 2.0).then(&translation(3.0, 4.0, -4.0)));
    cyl5.material.color = color(0.7922, 0.80, 0.8078);
    cyl5.material.diffuse = 0.3;
    cyl5.material.specular = 0.8;
    cyl5.material.shininess = 100.0;
    cyl5.material.shininess = 10.0;
    cyl5.material.reflective = 0.5;
    w.add_object(cyl5);

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
