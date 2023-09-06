use anyhow::Result;
use clap::Parser;
use rust_rtc::camera::render;
use rust_rtc::transformations::{translation, view_transform};
use rust_rtc::tuples::{point, vector};
use rust_rtc::utils;
use rust_rtc::utils::{parse_filename, RenderOptions};
use rust_rtc::world_loader::load_world;
use std::path::Path;

#[derive(Parser)]
pub struct Cli {
    /// World JSON5 filename (or use - for stdin)
    #[arg(short = 'i', long = "input", value_name = "FILE")]
    #[arg(value_parser = parse_filename)]
    pub input: String,

    #[clap(flatten)]
    pub common: utils::CommonArgs,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let (mut world, render_options) = load_world(Path::new(&cli.input))?;

    println!("{:#?}", world);
    println!("{:#?}", render_options);

    let options = render_options[0];

    // let options = RenderOptions {
    //     camera_transform: view_transform(
    //         &point(0.0, 1.5, -5.0),
    //         //&point(0.0, 0.5, 0.0),
    //         &point(0.0, 1.0, 0.0),
    //         &vector(0.0, 1.0, 0.0),
    //     ),
    //     //.then(&translation(0.0, 0.0, -20.0)),
    //     ..Default::default()
    // };

    // TODO:
    //  .
    //  - Handle cmd-line/json defaults properly.
    //    Cmd-line should override json.
    //  .
    //  - Handle multiple cameras.
    //    Each has a name, select first by default,
    //    but allow others to be used by name or index.
    //  .
    //  - Port other scenes to JSON5.

    utils::render_world(&world, options, &cli.common)?;

    Ok(())
}
