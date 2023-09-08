use anyhow::Result;
use clap::Parser;
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

    // TODO:
    //  .
    //  - Handle multiple cameras.
    //    Each has a name, select first by default,
    //    but allow others to be used by name or index.
    //  .
    //  - Port other scenes to JSON5.

    utils::render_world(&world, options, &cli.common)?;

    Ok(())
}
