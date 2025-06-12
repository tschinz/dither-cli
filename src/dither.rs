use std::path::PathBuf;

use image::{ExtendedColorType, ImageReader};

use crate::palette::{Color, ColorPalette, PALETTE_8C, PALETTE_16C, PALETTE_MONOCHROME, map_to_palette};

#[derive(clap::ValueEnum, Copy, Clone, Debug, Default)]
pub enum DitherMethod {
  None,
  #[default]
  FloydSteinberg,
  Jarvis,
  Judice,
  Ninke,
  Stucki,
  Atkinson,
  Burkes,
  Sierra,
  TwoRowSierra,
  SierraLite,
  Bayer4x4,
  Bayer8x8,
}

pub struct QuantizationError {
  pub r: f32,
  pub g: f32,
  pub b: f32,
}

pub const FLOYD_STEINBERG: [f32; 6] = [0.0, 0.0, 7.0 / 16.0, 3.0 / 16.0, 5.0 / 16.0, 1.0 / 16.0];

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
    DitherMethod::Jarvis => unimplemented!(),
    DitherMethod::Judice => unimplemented!(),
    DitherMethod::Ninke => unimplemented!(),
    DitherMethod::Stucki => unimplemented!(),
    DitherMethod::Atkinson => unimplemented!(),
    DitherMethod::Burkes => unimplemented!(),
    DitherMethod::Sierra => unimplemented!(),
    DitherMethod::TwoRowSierra => unimplemented!(),
    DitherMethod::SierraLite => unimplemented!(),
    DitherMethod::Bayer4x4 => unimplemented!(),
    DitherMethod::Bayer8x8 => unimplemented!(),
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
