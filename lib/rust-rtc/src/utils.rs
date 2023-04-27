use crate::camera::{camera, Resolution};
use crate::canvas::{ppm_from_canvas, Canvas};
use crate::math::MAX_RECURSIVE_DEPTH;
use crate::matrices::{identity4, Matrix4};
use crate::world::World;
use clap::{Parser, Args, ValueEnum};
use std::f64::consts::PI;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

#[derive(Args)]
#[clap(author, version, about, long_about = None)]
pub struct RenderArgs {
    /// Optional output filename (omit, or use - for stdout)
    #[arg(
        short = 'o',
        long = "output",
        value_name = "FILE",
        default_value = "image.ppm"
    )]
    #[arg(value_parser = parse_filename)]
    pub output: String,

    /// Pre-defined resolutions
    #[arg(short = 'r', long = "resolution", value_enum)]
    pub resolution: Option<Resolutions>,

    /// Generate low-resolution draft in selected aspect ratio
    #[arg(short = 'j', long = "draft")]
    pub draft: bool,

    /// Custom horizontal size (width) in pixels
    #[arg(short = 'x', long = "hsize")]
    #[arg(value_parser = clap::value_parser!(u32).range(1..))]
    pub hsize: Option<u32>,

    /// Custom vertical size (height) in pixels
    #[arg(short = 'y', long = "vsize")]
    #[arg(value_parser = clap::value_parser!(u32).range(1..))]
    pub vsize: Option<u32>,

    /// Maximum recursive depth
    #[arg(short = 'd', long = "recurse-depth")]
    #[arg(default_value_t = MAX_RECURSIVE_DEPTH)]
    pub max_recursive_depth: i32,

    /// Number of vertical subimage divisions, for multi-threaded rendering
    #[arg(short = 'm', long = "hdiv", default_value_t = 8)]
    #[arg(value_parser = clap::value_parser!(u32).range(1..))]
    pub hdiv: u32,

    /// Number of horizontal subimage divisions, for multi-threaded rendering
    #[arg(short = 'm', long = "vdiv", default_value_t = 8)]
    #[arg(value_parser = clap::value_parser!(u32).range(1..))]
    pub vdiv: u32,
}

fn parse_filename(name: &str) -> Result<String, String> {
    if name.is_empty() {
        Err(String::from("filename cannot be an empty string"))
    } else if name.trim().len() != name.len() {
        Err(String::from(
            "filename cannot have leading and trailing space",
        ))
    } else {
        Ok(name.to_string())
    }
}

#[derive(ValueEnum, Debug, Clone)]
#[clap(rename_all = "kebab_case")]
pub enum Resolutions {
    VGA,
    SVGA,
    XGA,
    SXGA,
    FHD,
    QHD,
    UHD,
    _4K,
}

#[derive(Args)]
pub struct CommonArgs {
    #[clap(flatten)]
    #[clap(next_help_heading = "Render Options")]
    pub render: RenderArgs,
}

#[derive(Parser)]
pub struct Cli {
    #[clap(flatten)]
    pub common: CommonArgs,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}

pub fn get_resolution(common_args: &CommonArgs, default: Resolution) -> Resolution {
    let base = match &common_args.render.resolution {
        Some(Resolutions::VGA) => Resolution::VGA,
        Some(Resolutions::SVGA) => Resolution::SVGA,
        Some(Resolutions::XGA) => Resolution::XGA,
        Some(Resolutions::SXGA) => Resolution::SXGA,
        Some(Resolutions::FHD) => Resolution::FHD,
        Some(Resolutions::QHD) => Resolution::QHD,
        Some(Resolutions::UHD) => Resolution::UHD_4K,
        Some(Resolutions::_4K) => Resolution::UHD_4K,
        _ => default,
    };

    let base = match common_args.render.hsize {
        Some(h) => Resolution { hsize: h, ..base },
        _ => base,
    };

    let base = match common_args.render.vsize {
        Some(v) => Resolution { vsize: v, ..base },
        _ => base,
    };

    if common_args.render.draft {
        Resolution {
            hsize: base.hsize / 4,
            vsize: base.vsize / 4,
        }
    } else {
        base
    }
}

#[derive(Copy, Clone)]
pub struct RenderOptions {
    pub default_resolution: Resolution,
    pub field_of_view: f64,
    pub camera_transform: Matrix4,
}

impl Default for RenderOptions {
    fn default() -> Self {
        RenderOptions {
            default_resolution: Resolution::VGA,
            field_of_view: PI / 3.0,
            camera_transform: identity4(),
        }
    }
}

pub fn render_world(world: &World, options: RenderOptions, common_args: &CommonArgs) -> Result<Canvas, io::Error> {
    let resolution = get_resolution(common_args, options.default_resolution);

    let pb = indicatif::ProgressBar::new(resolution.num_pixels());
    pb.set_style(
        indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {wide_bar:.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .expect("style should be valid"),
    );

    let mut cam = camera(resolution, options.field_of_view);

    let pb_update = Box::new(|x| { pb.inc(x); });

    cam.set_transform(&options.camera_transform);

    pb.set_message("Rendering...");

    let canvas = if common_args.render.hdiv == 1 && common_args.render.vdiv == 1 {
        cam.render_single_threaded(world, common_args.render.max_recursive_depth, Some(pb_update))
    } else {
        cam.render(world, common_args.render.max_recursive_depth, common_args.render.hdiv, common_args.render.vdiv, Some(pb_update))
    };

    pb.finish_with_message("Writing...");

    write_canvas(&canvas, &common_args.render.output)?;
    pb.finish_with_message("Complete");

    Ok(canvas)
}

pub fn write_canvas(canvas: &Canvas, output_filename: &str) -> io::Result<()> {
    let ppm = ppm_from_canvas(canvas);

    let mut out_writer = match output_filename {
        "-" => Box::new(io::stdout()) as Box<dyn Write>,
        x => {
            let path = Path::new(x);
            Box::new(File::create(path)?) as Box<dyn Write>
        }
    };

    out_writer.write_all(ppm.as_bytes())?;
    Ok(())
}
