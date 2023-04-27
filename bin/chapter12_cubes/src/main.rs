use rust_rtc::colors::{color, WHITE};
use rust_rtc::lights::point_light;
use rust_rtc::materials::{default_material, Material};

use rust_rtc::shapes::{cube, sphere, Shape};
use rust_rtc::transformations::{
    rotation_x, rotation_y, translate_x, translate_z, translation, uniform_scaling, view_transform,
};
use rust_rtc::tuples::{point, vector, Point};
use rust_rtc::utils;
use rust_rtc::utils::RenderOptions;
use rust_rtc::world::world;
use std::f64::consts::PI;
use std::process::ExitCode;
use clap::{Parser};

fn cube_grid(
    origin: Point,
    size: f64,
    mat: &Material,
    nx: i32,
    ny: i32,
    nz: i32,
    sep: f64,
) -> Vec<Shape> {
    let mut objects: Vec<Shape> = vec![];
    for x in 0..nx {
        for y in 0..ny {
            for z in 0..nz {
                let mut cube = cube();
                cube.set_transform(
                    &uniform_scaling(size)
                        .then(&translation(origin.x(), origin.y(), origin.z()))
                        .then(&translation(x as f64 * sep, y as f64 * sep, z as f64 * sep)),
                );
                cube.material = mat.clone();
                objects.push(cube);
            }
        }
    }
    objects
}

#[derive(Parser)]
pub struct Cli {
    /// Max number of lights in scene
    #[arg(short = 'l', long = "max-lights")]
    #[arg(value_parser = clap::value_parser!(u32).range(1..))]
    #[arg(default_value_t = 3)]
    pub max_lights: u32,

    #[clap(flatten)]
    pub common: utils::CommonArgs,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let mut w = world();

    let mut room = cube();
    room.set_transform(&uniform_scaling(200.0));
    room.material.color = color(0.8, 0.8, 1.0);
    room.material.diffuse = 0.3;
    room.material.ambient = 0.2;
    room.material.specular = 0.0;
    room.material.shininess = 1.0;
    room.material.casts_shadow = false;
    room.material.receives_shadow = false;
    w.add_object(room);

    let sep = 3.0;
    let rotation = PI / 6.0;

    let mut green_cube_mat = default_material();
    green_cube_mat.color = color(0.0, 0.8, 0.1);
    green_cube_mat.specular = 0.4;
    //green_cube_mat.transparency = 0.8;
    green_cube_mat.reflective = 0.7;

    let green_cubes = cube_grid(
        point(4.0, 0.0, 5.0),
        1.3,
        &green_cube_mat,
        3,
        3,
        3,
        sep * 1.3,
    );
    for mut cube in green_cubes {
        let mut t = *cube.transform();
        cube.set_transform(&t.then(&rotation_y(rotation)));
        w.add_object(cube);
    }

    let ball_scale = 18.0;
    let mut ball = sphere(1);
    ball.set_transform(
        &uniform_scaling(ball_scale)
            .then(&translate_z(-3.0))
            .then(&translate_x(-24.0))
            .then(&rotation_y(rotation)),
    );
    ball.material.color = WHITE;
    ball.material.diffuse = 0.1;
    ball.material.specular = 1.0;
    ball.material.shininess = 1000.0;
    ball.material.reflective = 0.5;
    w.add_object(ball);

    let mut red_cube_mat = default_material();
    red_cube_mat.color = color(0.9, 0.0, 0.1);
    red_cube_mat.diffuse = 0.7;
    red_cube_mat.reflective = 0.7;

    let red_cubes = cube_grid(point(4.0, -12.0, -10.0), 1.0, &red_cube_mat, 4, 4, 4, sep);
    for mut cube in red_cubes {
        let mut t = *cube.transform();
        cube.set_transform(&t.then(&rotation_y(rotation)));
        w.add_object(cube);
    }

    let mut blue_cube = cube();
    blue_cube.set_transform(
        &uniform_scaling(2.0)
            .then(&rotation_y(PI / 6.0))
            .then(&rotation_x(PI / 4.0))
            .then(&translation(0.0, 4.0, -5.0)),
    );
    blue_cube.material.color = color(0.0, 0.1, 0.8);
    blue_cube.material.reflective = 0.7;
    w.add_object(blue_cube);

    let dimming = cli.max_lights as f64;
    w.add_light(point_light(point(-2.0, 10.0, -10.0), color(1.0, 1.0, 1.0) / dimming));
    if cli.max_lights > 1 {
        w.add_light(point_light(point(12.0, 10.0, -10.0), color(1.0, 1.0, 1.0) / dimming));
    }
    if cli.max_lights > 2 {
        w.add_light(point_light(point(12.0, 20.0, -20.0), color(1.0, 1.0, 1.0) / dimming));
    }

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
