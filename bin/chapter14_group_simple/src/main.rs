use rust_rtc::colors::{BLACK, color, WHITE};
use rust_rtc::lights::point_light;

use rust_rtc::shapes::{group, sphere, plane, cylinder, infinite_cylinder};
use rust_rtc::transformations::{rotation_x, rotation_y, rotation_z, translate_x, translate_y, translate_z, translation, view_transform};
use rust_rtc::tuples::{point, vector};
use rust_rtc::utils;
use rust_rtc::utils::RenderOptions;
use rust_rtc::world::world;
use std::f64::consts::PI;
use std::process::ExitCode;
use rust_rtc::patterns::{checkers_pattern, solid_pattern};

fn main() -> ExitCode {
    let cli = utils::parse_args();

    let mut w = world();

    let mut ptn = checkers_pattern(solid_pattern(&BLACK), solid_pattern(&WHITE));

    let mut pxz = plane();
    let mut pxy = pxz.clone();
    let mut pyz = pxz.clone();

    let mut ptn_xz = ptn.clone();
    ptn_xz.set_transform(&translate_x(1.0));
    pxz.material.set_pattern(&ptn_xz);

    pxy.material.set_pattern(&ptn);
    pyz.material.set_pattern(&ptn);

    //w.add_object(pxz);

    pxy.set_transform(&rotation_x(PI / 2.0));
    //w.add_object(pxy);

    pyz.set_transform(&rotation_z(-PI / 2.0));
    //w.add_object(pyz);

    // Group 1
    let mut s1 = sphere(1);
    s1.set_transform(&translation(0.75, 0.0, 0.0));
    let s1_idx = w.add_object(s1);
    let mut s2 = sphere(2);
    s2.set_transform(&translation(0.0, 0.0, 0.75));
    let s2_idx = w.add_object(s2);

    let mut g1 = group();
    //g1.set_transform(&translation(0.0, 3.0, 0.0));
    g1.set_transform(&translation(1.0, 1.0, -1.0));

    let g1_idx = w.add_object(g1);
    println!("{g1_idx:?}");
    w.add_child(&g1_idx, &s1_idx).unwrap();
    w.add_child(&g1_idx, &s2_idx).unwrap();

    w.add_light(point_light(point(-2.0, 5.0, -10.0), color(1.0, 1.0, 1.0)));

    let options = RenderOptions {
        camera_transform: view_transform(
            &point(-5.0, 1.5, -5.0),
            &point(0.0, 0.5, 0.0),
            &vector(0.0, 1.0, 0.0),
        ),
        //.then(&translation(-15.0, 0.0, -15.0)),
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
