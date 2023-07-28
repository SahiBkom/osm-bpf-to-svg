use anyhow::Result;
use clap::Parser;
use log::*;
use osm_bpf_to_svg::SelectBox;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of input file
    input: String,

    /// Name of the prson to greet
    x: u32,

    /// Name of the person to greet
    y: u32,

    /// Name of the person to greet
    #[arg(default_value_t = 1000)]
    w: u32,

    /// height
    #[arg(default_value_t = 1000)]
    h: u32,

    /// Name of output
    #[arg(short, long, default_value = "out.svg")]
    output: String,

    /// Name of style file
    #[arg(short, long)]
    style: Option<String>,
}

impl Args {}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();
    debug!("args {:?}", args);
    let select_box = SelectBox::new(args.x, args.y, args.w, args.h);
    osm_bpf_to_svg::doit(select_box, args.input, args.output, args.style)?;

    Ok(())
}
