use crate::dither::DitherMethod;
use crate::palette::ColorPalette;
use clap::Parser;
use std::path::PathBuf;

/// rust dither CLI is a simple command-line tool for dithering images
///
/// Rust dither CLI
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
  #[clap(short, long = "in")]
  pub in_img: PathBuf,

  #[clap(short, long = "out", default_value = "out.png")]
  pub out_img: Option<PathBuf>,

  #[clap(short, long = "dither", default_value_t, value_enum)]
  pub dither_type: DitherMethod,

  #[clap(short, long = "color", default_value_t, value_enum)]
  pub color_palette: ColorPalette,
}
