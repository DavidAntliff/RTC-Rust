use crate::camera::{camera, render, Resolution};
use crate::canvas::{Canvas, ppm_from_canvas};
use crate::matrices::Matrix4;
use crate::world::World;

pub fn render_world(
              world: &World,
              resolution: Resolution,
              field_of_view: f64,
              camera_transform: Matrix4,
              recursive_depth: i32) -> Canvas {

    let pb = indicatif::ProgressBar::new(resolution.num_pixels());
    pb.set_style(indicatif::ProgressStyle::with_template(
        "[{elapsed_precise}] {wide_bar:.cyan/blue} {pos:>7}/{len:7} {msg}")
        .expect("style should be valid"));

    let mut cam = camera(resolution, field_of_view);

    cam.set_progress_callback(Box::new(|x| { pb.inc(x); }));

    cam.set_transform(&camera_transform);

    pb.set_message("Rendering...");
    let canvas = render(&mut cam,  world, recursive_depth);
    pb.finish_with_message("Writing...");
    let ppm = ppm_from_canvas(&canvas);
    print!("{}", ppm);
    pb.finish_with_message("Complete");

    canvas
}
