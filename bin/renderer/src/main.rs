use anyhow::Result;
use clap::Parser;
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

    let mut w = load_world(Path::new(&cli.input))?;

    let options = RenderOptions {
        camera_transform: view_transform(
            &point(0.0, 1.5, -5.0),
            //&point(0.0, 0.5, 0.0),
            &point(0.0, 1.0, 0.0),
            &vector(0.0, 1.0, 0.0),
        ),
        //.then(&translation(0.0, 0.0, -20.0)),
        ..Default::default()
    };

    utils::render_world(&w, options, &cli.common)?;

    Ok(())
}
