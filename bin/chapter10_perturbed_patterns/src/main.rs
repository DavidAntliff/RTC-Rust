use rust_rtc::camera::{Resolution};

use rust_rtc::colors::{color, colori, RED, WHITE};
use rust_rtc::lights::point_light;
use rust_rtc::materials::default_material;
use rust_rtc::math::MAX_RECURSIVE_DEPTH;
use rust_rtc::patterns::{
    blended_pattern, gradient_pattern, perturbed_pattern, ring_pattern, stripe_pattern,
};
use rust_rtc::shapes::{plane, sphere};
use rust_rtc::transformations::{
    rotation_x, rotation_y, rotation_z, scaling, translation, view_transform,
};
use rust_rtc::tuples::{point, vector};
use rust_rtc::world::world;
use std::f64::consts::PI;

fn main() {
    let perturb_factor = 1.0;

    let mut w = world();

    let mut floor = plane();
    floor.material = default_material();
    floor.material.color = color(1.0, 0.9, 0.9);
    floor.material.ambient = 0.2;
    floor.material.specular = 0.0;
    let mut floor_pattern_1 =
        perturbed_pattern(&stripe_pattern(&RED, &WHITE), 2.0 * perturb_factor, 4, 0.9);
    let mut floor_pattern_2 =
        perturbed_pattern(&stripe_pattern(&RED, &WHITE), 2.0 * perturb_factor, 4, 0.9);
    let scale = 0.4;
    floor_pattern_1.set_transform(&scaling(scale, scale, scale).then(&rotation_y(PI / 4.0)));
    floor_pattern_2.set_transform(&scaling(scale, scale, scale).then(&rotation_y(-PI / 4.0)));
    let mut floor_pattern = blended_pattern(&floor_pattern_1, &floor_pattern_2);
    floor_pattern.set_transform(&rotation_y(-PI / 8.0));
    floor.material.set_pattern(&floor_pattern);
    w.add_object(floor);

    let mut middle = sphere(4);
    middle.set_transform(&translation(-0.5, 1.0, 0.5));
    middle.material = default_material();
    middle.material.color = color(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.9;
    middle.material.specular = 0.7;
    let mut middle_pattern = perturbed_pattern(
        &stripe_pattern(&colori(13, 104, 53), &colori(15, 158, 79)),
        2.0 * perturb_factor,
        3,
        0.8,
    );
    middle_pattern.set_transform(
        &scaling(0.25, 0.25, 0.25)
            .then(&rotation_z(-PI / 4.0))
            .then(&rotation_y(-PI / 4.0)),
    );
    middle.material.set_pattern(&middle_pattern);
    w.add_object(middle);

    let mut right = sphere(5);
    right.set_transform(&scaling(0.5, 0.5, 0.5).then(&translation(1.5, 0.5, -0.5)));
    right.material = default_material();
    right.material.color = color(0.5, 1.0, 0.1);
    right.material.diffuse = 0.9;
    right.material.specular = 0.3;
    let mut right_pattern = perturbed_pattern(
        &gradient_pattern(&colori(200, 40, 0), &colori(200, 180, 0)),
        0.8 * perturb_factor,
        4,
        0.9,
    );
    right_pattern.set_transform(
        &scaling(2.2, 2.2, 2.2)
            .then(&rotation_z(PI / 6.0))
            .then(&translation(2.0, 0.0, 0.0)),
    );
    right.material.set_pattern(&right_pattern);
    w.add_object(right);

    let mut left = sphere(6);
    left.set_transform(&scaling(0.33, 0.33, 0.33).then(&translation(-1.5, 0.33, -0.75)));
    left.material = default_material();
    left.material.color = color(1.0, 0.8, 0.1);
    left.material.diffuse = 0.9;
    left.material.specular = 0.3;
    let mut left_pattern = perturbed_pattern(
        &ring_pattern(&colori(199, 240, 194), &colori(95, 191, 95)),
        1.5 * perturb_factor,
        4,
        0.9,
    );
    left_pattern.set_transform(
        &scaling(0.3, 0.3, 0.3)
            .then(&rotation_x(-PI / 3.0))
            .then(&rotation_y(-0.2)),
    );
    left.material.set_pattern(&left_pattern);
    w.add_object(left);

    w.add_light(point_light(point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0)));

    //let resolution = Resolution::VGA;  // 640 x 480
    //let resolution = Resolution::XGA;  // 1024 x 768
    let resolution = Resolution::QHD; // 2560 x 1440
                                      //let resolution = Resolution::UHD_4K;  // 3840 x 2160

    let camera_transform = view_transform(
        &point(0.0, 1.5, -5.0),
        &point(0.0, 1.0, 0.0),
        &vector(0.0, 1.0, 0.0),
    );

    rust_rtc::utils::render_world(
        &w,
        resolution,
        PI / 3.0,
        camera_transform,
        MAX_RECURSIVE_DEPTH,
    );
}
