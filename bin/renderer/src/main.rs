use anyhow::{Context, Result};
use clap::Parser;
use rust_rtc::utils;
use rust_rtc::utils::parse_filename;
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

    let (world, render_options) = load_world(Path::new(&cli.input))?;

    println!("{:#?}", world);
    println!("{:#?}", render_options);

    let options = render_options
        .get(&cli.common.render.camera_name)
        .context("No camera")?;

    // TODO:
    //  - Port other scenes to JSON5.

    utils::render_world(&world, *options, &cli.common)?;

    Ok(())
}
