use std::path::PathBuf;

use image::{ExtendedColorType, ImageReader};

use crate::palette::{Color, ColorPalette, PALETTE_8C, PALETTE_16C, PALETTE_MONOCHROME, map_to_palette};

#[derive(clap::ValueEnum, Copy, Clone, Debug, Default)]
pub enum DitherMethod {
  None,
  #[default]
  FloydSteinberg,
  Simple2D,
  Jarvis,
  Atkinson,
  Stucki,
  Burkes,
  Sierra,
  TwoRowSierra,
  SierraLite,
  Bayer2x2,
  Bayer4x4,
  Bayer8x8,
}

pub struct QuantizationError {
  pub r: f32,
  pub g: f32,
  pub b: f32,
}

pub const FLOYD_STEINBERG: [f32; 6] = [0.0, 0.0, 7.0 / 16.0, 3.0 / 16.0, 5.0 / 16.0, 1.0 / 16.0];
pub const JARVIS: [f32; 15] = [
  0.0,
  0.0,
  0.0,
  7.0 / 48.0,
  5.0 / 48.0,
  3.0 / 48.0,
  5.0 / 48.0,
  7.0 / 48.0,
  5.0 / 48.0,
  3.0 / 48.0,
  1.0 / 48.0,
  3.0 / 48.0,
  5.0 / 48.0,
  3.0 / 48.0,
  1.0 / 48.0,
];
// Bayer(n)=( 4⋅Bayer(n−1)+0 4⋅Bayer(n−1)+2 )
//            4⋅Bayer(n−1)+3 4⋅Bayer(n−1)+1
// Bayer(0)
pub const BAYER2X2: [f32; 4] = [0.0, 2.0 / 4.0, 3.0 / 4.0, 1.0 / 4.0];
// Bayer(1)
pub const BAYER4X4: [f32; 16] = [
  0.0,
  8.0 / 16.0,
  2.0 / 16.0,
  10.0 / 16.0,
  12.0 / 16.0,
  4.0 / 16.0,
  14.0 / 16.0,
  6.0 / 16.0,
  3.0 / 16.0,
  11.0 / 16.0,
  1.0 / 16.0,
  9.0 / 16.0,
  15.0 / 16.0,
  7.0 / 16.0,
  13.0 / 16.0,
  5.0 / 16.0,
];
// Bayer(2)
pub const BAYER8X8: [f32; 64] = [
  0.0,
  32.0 / 64.0,
  8.0 / 64.0,
  40.0 / 64.0,
  2.0 / 64.0,
  34.0 / 64.0,
  10.0 / 64.0,
  42.0 / 64.0,
  48.0 / 64.0,
  16.0 / 64.0,
  56.0 / 64.0,
  24.0 / 64.0,
  50.0 / 64.0,
  18.0 / 64.0,
  58.0 / 64.0,
  26.0 / 64.0,
  12.0 / 64.0,
  44.0 / 64.0,
  4.0 / 64.0,
  36.0 / 64.0,
  14.0 / 64.0,
  46.0 / 64.0,
  6.0 / 64.0,
  38.0 / 64.0,
  60.0 / 64.0,
  28.0 / 64.0,
  52.0 / 64.0,
  20.0 / 64.0,
  62.0 / 64.0,
  30.0 / 64.0,
  54.0 / 64.0,
  22.0 / 64.0,
  3.0 / 64.0,
  35.0 / 64.0,
  11.0 / 64.0,
  43.0 / 64.0,
  1.0 / 64.0,
  33.0 / 64.0,
  9.0 / 64.0,
  41.0 / 64.0,
  51.0 / 64.0,
  19.0 / 64.0,
  59.0 / 64.0,
  27.0 / 64.0,
  49.0 / 64.0,
  17.0 / 64.0,
  57.0 / 64.0,
  25.0 / 64.0,
  15.0 / 64.0,
  47.0 / 64.0,
  7.0 / 64.0,
  39.0 / 64.0,
  13.0 / 64.0,
  45.0 / 64.0,
  5.0 / 64.0,
  37.0 / 64.0,
  63.0 / 64.0,
  31.0 / 64.0,
  55.0 / 64.0,
  23.0 / 64.0,
  61.0 / 64.0,
  29.0 / 64.0,
  53.0 / 64.0,
  21.0 / 64.0,
];

pub const SIMPLE2D: [f32; 4] = [0.0, 0.5, 0.5, 0.0];

pub const ATKINSON: [f32; 12] = [0.0, 0.0, 1.0 / 8.0, 1.0 / 8.0, 1.0 / 8.0, 1.0 / 8.0, 1.0 / 8.0, 0.0, 0.0, 1.0 / 8.0, 0.0, 0.0];

pub const STUCKI: [f32; 15] = [
  0.0,
  0.0,
  0.0,
  8.0 / 42.0,
  4.0 / 42.0,
  2.0 / 42.0,
  4.0 / 42.0,
  8.0 / 42.0,
  4.0 / 42.0,
  2.0 / 42.0,
  1.0 / 42.0,
  2.0 / 42.0,
  4.0 / 42.0,
  2.0 / 42.0,
  1.0 / 42.0,
];

pub const BURKES: [f32; 10] = [
  0.0,
  0.0,
  0.0,
  8.0 / 32.0,
  4.0 / 32.0,
  2.0 / 32.0,
  4.0 / 32.0,
  8.0 / 32.0,
  4.0 / 32.0,
  2.0 / 32.0,
];

pub const SIERRA: [f32; 15] = [
  0.0,
  0.0,
  0.0,
  5.0 / 32.0,
  3.0 / 32.0,
  2.0 / 32.0,
  4.0 / 32.0,
  5.0 / 32.0,
  4.0 / 32.0,
  2.0 / 32.0,
  0.0,
  2.0 / 32.0,
  3.0 / 32.0,
  2.0 / 32.0,
  0.0,
];
pub const TWOROWSIERRA: [f32; 10] = [
  0.0,
  0.0,
  0.0,
  4.0 / 16.0,
  3.0 / 16.0,
  1.0 / 16.0,
  2.0 / 16.0,
  3.0 / 16.0,
  2.0 / 16.0,
  1.0 / 16.0,
];
pub const SIERRALITE: [f32; 6] = [0.0, 0.0, 2.0 / 4.0, 1.0 / 4.0, 1.0 / 4.0, 0.0];

pub fn open_image(path: &PathBuf) -> (Vec<u8>, u32, u32) {
  //let image = ImageReader::open(path).unwrap().decode().unwrap().into_rgba8();
  let image = ImageReader::open(path).unwrap().decode().unwrap().into_rgb8();

  let (width, height) = image.dimensions();
  let buffer = image.into_raw();
  (buffer, width, height)
}

pub fn save_image(buffer: Vec<u8>, path: PathBuf, width: u32, height: u32) {
  let _ = image::save_buffer(path, &buffer, width, height, ExtendedColorType::Rgb8);
}

pub fn dither(buffer: &mut Vec<u8>, dither_type: DitherMethod, color_palette: ColorPalette, width: u32, height: u32) {
  // This function will implement the dithering logic based on the dither_type and color_palette.
  // For now, we will just print the parameters to demonstrate the function call.
  //println!("Dithering with method: {:?} and color palette: {:?}", dither_type, color_palette);

  // get the color palette as slice
  let color_palette = match color_palette {
    ColorPalette::Monochrome => &PALETTE_MONOCHROME[..],
    ColorPalette::COLOR8 => &PALETTE_8C[..],
    ColorPalette::COLOR16 => &PALETTE_16C[..],
  };

  // get the dither method weights
  let weights: &[f32] = match dither_type {
    DitherMethod::None => return, // No dithering
    DitherMethod::FloydSteinberg => &FLOYD_STEINBERG,
    DitherMethod::Simple2D => &SIMPLE2D,
    DitherMethod::Jarvis => &JARVIS,
    DitherMethod::Atkinson => &ATKINSON,
    DitherMethod::Stucki => &STUCKI,
    DitherMethod::Burkes => &BURKES,
    DitherMethod::Sierra => &SIERRA,
    DitherMethod::TwoRowSierra => &TWOROWSIERRA,
    DitherMethod::SierraLite => &SIERRALITE,
    DitherMethod::Bayer2x2 => &BAYER2X2,
    DitherMethod::Bayer4x4 => &BAYER4X4,
    DitherMethod::Bayer8x8 => &BAYER8X8,
  };
  // map the color palette to the buffer
  for cy in 0..height {
    for cx in 0..width {
      let i = ((cy * width + cx) * 3) as usize; // Assuming RGB format
      let (new_color, qe) = map_to_palette(Color::from(&buffer[i..i + 3]), &color_palette);
      buffer[i] = new_color.r;
      buffer[i + 1] = new_color.g;
      buffer[i + 2] = new_color.b;

      // apply the quanization error accross the neighboring pixels
      for dy in 0..=1 {
        for dx in -1..=1 {
          let x = cx as isize + dx;
          let y = cy + dy;
          if x < 0 || x >= width as isize || y >= height {
            continue; // Skip out of bounds
          }
          // calculated index for the neighboring pixel
          let i = ((y * width + x as u32) * 3) as usize;
          // Calculate the quantization table index
          let di = ((dy * 3) + (1 as isize + dx) as u32) as usize;
          // Apply the quantization error to the neighboring pixel
          buffer[i] = (buffer[i] as f32 + (qe.r * weights[di])).round().clamp(0.0, 255.0) as u8;
          buffer[i + 1] = (buffer[i + 1] as f32 + (qe.g * weights[di])).round().clamp(0.0, 255.0) as u8;
          buffer[i + 2] = (buffer[i + 2] as f32 + (qe.b * weights[di])).round().clamp(0.0, 255.0) as u8;
        }
      }
    }
  }
}
