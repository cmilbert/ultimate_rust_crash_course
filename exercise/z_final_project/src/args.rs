// use clap::{App, Arg};
use clap::Parser;

/// Simple image modificaiton program
#[derive(Parser, Debug)]
#[clap(author = "Chris Milbert", version = "0.1", about, long_about = None)]
pub struct Args {
   /// Input file to transform
    #[clap(required = true, value_parser, index = 1)]
    pub infile: String,

    /// Output file for transformation
    #[clap(required = true, value_parser, index = 2)]    
    pub outfile: String,

    /// Blur amount
    #[clap(short, long, value_parser, default_value_t = 0.0)]
    pub blur: f32,
    
    /// Brighten amount
    #[clap(short = 'l', long, value_parser, default_value_t = 0)]
    pub brighten: i32,
    
    /// Generate a fractal
    #[clap(long)]
    pub fractal: bool,

    /// Generate a fun image
    #[clap(long)]
    pub generate: bool,

    /// Inverts an image
    #[clap(short, long)]
    pub invert: bool,

    /// Converts an image to greyscale
    #[clap(short, long)]
    pub grayscale: bool,

    /// Rotate image by degrees
    #[clap(short, long, value_parser, default_value_t = 0.0)]
    pub rotation: f32,

    /// Crop x offset
    #[clap(short, long, value_parser, default_value_t = 0)]
    pub x_offset: u32,
    
    /// Crop y offset
    #[clap(short, long, value_parser, default_value_t = 0)]
    pub y_offset: u32,

    /// Crop width
    #[clap(short, long, value_parser, default_value_t = 0)]
    pub width: u32,

    /// Crop height
    #[clap(short, long, value_parser, default_value_t = 0)]
    pub height: u32,
}
