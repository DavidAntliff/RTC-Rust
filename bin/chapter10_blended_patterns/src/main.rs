use rust_rtc::camera::{camera, render};
use rust_rtc::canvas::ppm_from_canvas;
use rust_rtc::colors::{color, colori, Color, WHITE};
use rust_rtc::lights::point_light;
use rust_rtc::materials::default_material;
use rust_rtc::math::MAX_RECURSIVE_DEPTH;
use rust_rtc::patterns::{blended_pattern, ring_pattern, stripe_pattern};
use rust_rtc::shapes::{plane, sphere};
use rust_rtc::transformations::{rotation_x, rotation_y, scaling, translation, view_transform};
use rust_rtc::tuples::{point, vector};
use rust_rtc::world::world;
use std::f64::consts::PI;

fn main() {
    let mut w = world();

    let color1: Color = WHITE;
    let color2: Color = colori(40, 99, 40);
    let color3: Color = colori(167, 83, 104);
    let color4: Color = colori(124, 41, 62);
    //let color5: Color = colori(63, 63, 63);
    //let color6: Color = colori(104, 104, 104);

    let mut floor = plane();
    floor.material = default_material();
    floor.material.diffuse = 1.0;
    floor.material.specular = 0.0;
    let scale = 0.5;
    let mut floor_pattern_1 = stripe_pattern(&color1, &color2);
    let mut floor_pattern_2 = stripe_pattern(&color1, &color2);
    floor_pattern_1.set_transform(&scaling(scale, scale, scale).then(&rotation_y(PI / 4.0)));
    floor_pattern_2.set_transform(&scaling(scale, scale, scale).then(&rotation_y(-PI / 4.0)));
    let floor_pattern = blended_pattern(&floor_pattern_1, &floor_pattern_2);
    floor.material.set_pattern(&floor_pattern);
    w.add_object(floor);

    let mut middle = sphere(4);
    middle.set_transform(&translation(-0.5, 1.0, 0.5));
    middle.material = default_material();
    middle.material.color = color(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.8;
    middle.material.specular = 0.6;
    middle.material.shininess = 100.0;
    let mut middle_pattern_1 = ring_pattern(&color1, &color3);
    middle_pattern_1.set_transform(&scaling(0.2, 0.2, 0.2));
    let mut middle_pattern_2 = ring_pattern(&color1, &color4);
    middle_pattern_2.set_transform(&scaling(0.2, 0.2, 0.2).then(&rotation_x(-PI / 4.0)));
    let mut middle_pattern = blended_pattern(&middle_pattern_1, &middle_pattern_2);
    middle_pattern.set_transform(&rotation_y(PI / 4.0).then(&rotation_x(-PI / 4.0)));
    middle.material.set_pattern(&middle_pattern);
    w.add_object(middle);

    w.add_light(point_light(point(10.0, 20.0, -10.0), color(1.0, 1.0, 1.0)));

    //let mut cam = camera(100, 50, PI / 3.0);
    //let mut cam = camera(1024, 768, PI / 3.0);
    //let mut cam = camera(1600, 800, PI / 3.0);
    let mut cam = camera(2048, 1536, PI / 3.0);
    //let mut cam = camera(3840, 2160, PI / 3.0);

    cam.set_transform(&view_transform(
        &point(0.0, 1.5, -5.0),
        &point(0.0, 1.0, 0.0),
        &vector(0.0, 1.0, 0.0),
    ));

    let canvas = render(&cam, &w, MAX_RECURSIVE_DEPTH);
    let ppm = ppm_from_canvas(&canvas);
    print!("{}", ppm);
}
