use rust_rtc::colors::{color, GREEN, GREY25, GREY75, RED, WHITE};
use rust_rtc::lights::point_light;
use rust_rtc::materials::{default_material, RefractiveIndex};
use rust_rtc::patterns::{blended_pattern, stripe_pattern};
use rust_rtc::shapes::{plane, sphere};
use rust_rtc::transformations::{rotation_y, scaling, translation, view_transform};
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
    //floor.set_transform(&rotation_x(-PI / 4.0).then(&translation(0.0, 0.0, 7.0)));
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

    // Large clear glass sphere
    let radius = 1.5;
    let mut sphere1 = sphere(1);
    sphere1.set_transform(&scaling(radius, radius, radius).then(&translation(1.1, radius, 0.0)));
    sphere1.material.color = WHITE;
    sphere1.material.diffuse = 0.1;
    sphere1.material.specular = 1.0;
    sphere1.material.shininess = 300.0;
    sphere1.material.reflective = 1.0; //0.99;
    sphere1.material.transparency = 1.0; //1;//0.99;
    sphere1.material.refractive_index = RefractiveIndex::GLASS;
    w.add_object(sphere1);

    // Air bubble in clear sphere
    let mut bubble = sphere(100);
    let radius = radius * 0.8;
    bubble.set_transform(&scaling(radius, radius, radius).then(&translation(1.1, radius, 0.0)));
    bubble.material.color = WHITE;
    bubble.material.ambient = 0.0;
    bubble.material.diffuse = 0.0;
    bubble.material.specular = 0.9;
    bubble.material.shininess = 300.0;
    bubble.material.reflective = 0.9;
    bubble.material.transparency = 0.9;
    bubble.material.refractive_index = RefractiveIndex::AIR;
    w.add_object(bubble);

    // Small red sphere
    let mut sphere2 = sphere(2);
    sphere2.set_transform(&scaling(0.4, 0.4, 0.4).then(&translation(-2.3, 0.4, -1.2)));
    sphere2.material.color = RED;
    sphere2.material.ambient = 0.15;
    sphere2.material.diffuse = 0.1;
    sphere2.material.specular = 0.7;
    sphere2.material.shininess = 100.0;
    sphere2.material.reflective = 1.0;
    sphere2.material.transparency = 1.0;
    sphere2.material.refractive_index = RefractiveIndex::GLASS;
    w.add_object(sphere2);

    // Small green sphere
    let mut sphere3 = sphere(3);
    sphere3.set_transform(&scaling(0.5, 0.5, 0.5).then(&translation(2.7, 0.5, -0.8)));
    sphere3.material.color = GREEN;
    sphere3.material.diffuse = 0.15;
    sphere3.material.specular = 0.7;
    sphere3.material.shininess = 100.0;
    sphere3.material.reflective = 1.0;
    sphere3.material.transparency = 1.0;
    sphere3.material.refractive_index = RefractiveIndex::GLASS;
    w.add_object(sphere3);

    // Tiny unreflective magenta sphere
    let mut sphere4 = sphere(4);
    sphere4.set_transform(&scaling(0.2, 0.2, 0.2).then(&translation(1.5, 0.2, -1.8)));
    sphere4.material.color = color(0.8, 0.0, 0.8);
    sphere4.material.specular = 0.5;
    sphere4.material.shininess = 10.0;
    sphere4.material.reflective = 0.0;
    w.add_object(sphere4);

    // Large blue sphere
    let mut sphere5 = sphere(5);
    sphere5.set_transform(&scaling(0.6, 0.6, 0.6).then(&translation(-1.0, 0.6, -0.8)));
    sphere5.material.color = color(0.1, 0.0, 1.0);
    sphere5.material.specular = 1.0;
    sphere5.material.shininess = 1000.0;
    sphere5.material.reflective = 0.8;
    sphere5.material.transparency = 0.0;
    sphere5.material.refractive_index = RefractiveIndex::GLASS;
    w.add_object(sphere5);

    // Huge gold sphere behind the camera
    let mut sphere6 = sphere(6);
    sphere6.set_transform(&scaling(2.0, 2.0, 2.0).then(&translation(-2.0, 2.0, 2.2)));
    sphere6.material.color = color(0.9, 0.7, 0.0);
    sphere6.material.specular = 0.9;
    sphere6.material.shininess = 500.0;
    sphere6.material.reflective = 0.3;
    w.add_object(sphere6);

    // Reflective silver sphere in the distance
    let mut sphere7 = sphere(7);
    sphere7.set_transform(&scaling(2.0, 2.0, 2.0).then(&translation(8.0, 2.0, 15.0)));
    sphere7.material.color = WHITE;
    sphere7.material.diffuse = 0.1;
    sphere7.material.specular = 1.0;
    sphere7.material.shininess = 1000.0;
    sphere7.material.reflective = 1.0;
    w.add_object(sphere7);

    w.add_light(point_light(point(5.0, 10.0, -8.0), color(0.9, 0.9, 0.9)));

    let options = RenderOptions {
        // From the front
        camera_transform: view_transform(
            &point(0.0, 2.5, -5.0),
            &point(0.0, 0.5, 5.0),
            &vector(0.0, 1.0, 0.0),
        )
        .then(&translation(0.0, 0.0, -2.5)),
        // From above:
        // let camera_transform: view_transform(
        //         &point(0.0, 2.5, 0.0),
        //         &point(0.0, 0.0, 0.5),
        //         &vector(0.0, 0.0, 1.0),
        //     ).then(&translation(-1.0, -1.0, -8.0)),
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
