use crate::dither::QuantizationError;

#[derive(clap::ValueEnum, Copy, Clone, Debug, Default)]
pub enum ColorPalette {
  #[default]
  Monochrome,
  COLOR8,
  COLOR16,
}

pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

impl From<u32> for Color {
  fn from(v: u32) -> Self {
    Color {
      r: ((v >> 16) & 0xFF) as u8,
      g: ((v >> 8) & 0xFF) as u8,
      b: (v & 0xFF) as u8,
    }
  }
}

impl From<&[u8]> for Color {
  fn from(v: &[u8]) -> Self {
    Color { r: v[0], g: v[1], b: v[2] }
  }
}

// searching the closest color in the palette
pub fn map_to_palette(orig_color: Color, palette: &[Color]) -> (&Color, QuantizationError) {
  // simple stupid linear search
  // this can be optimized with a better algorithm
  let mut min_distance = f32::INFINITY;
  let mut color = &palette[0];
  for c in palette {
    let distance =
      // sqrt not needed since we only compare distances, not actual values
      //((orig_color.r as f32 - c.r as f32).powi(2) + (orig_color.g as f32 - c.g as f32).powi(2) + (orig_color.b as f32 - c.b as f32).powi(2)).sqrt();
      (orig_color.r as f32 - c.r as f32).powi(2) + (orig_color.g as f32 - c.g as f32).powi(2) + (orig_color.b as f32 - c.b as f32).powi(2);
    if distance < min_distance {
      color = &c;
      min_distance = distance;
    }
  }
  let qe = QuantizationError {
    r: orig_color.r as f32 - color.r as f32,
    g: orig_color.g as f32 - color.g as f32,
    b: orig_color.b as f32 - color.b as f32,
  };

  (color, qe)
}

pub const PALETTE_16C: [Color; 16] = [
  //Color::from(0x000000), // does not work since its a const
  Color { r: 0x00, g: 0x00, b: 0x00 },
  Color { r: 0x9d, g: 0x9d, b: 0x9d },
  Color { r: 0xff, g: 0xff, b: 0xff },
  Color { r: 0xbe, g: 0x26, b: 0x33 },
  Color { r: 0xe0, g: 0x6f, b: 0x8b },
  Color { r: 0x49, g: 0x3c, b: 0x2b },
  Color { r: 0xa4, g: 0x64, b: 0x22 },
  Color { r: 0xeb, g: 0x89, b: 0x31 },
  Color { r: 0xf7, g: 0xe2, b: 0x6b },
  Color { r: 0x2f, g: 0x48, b: 0x4e },
  Color { r: 0x44, g: 0x89, b: 0x1a },
  Color { r: 0xa3, g: 0xce, b: 0x27 },
  Color { r: 0x1b, g: 0x26, b: 0x32 },
  Color { r: 0x00, g: 0x57, b: 0x84 },
  Color { r: 0x31, g: 0xa2, b: 0xf2 },
  Color { r: 0xb2, g: 0xdc, b: 0xef },
];

pub const PALETTE_8C: [Color; 8] = [
  Color { r: 0x00, g: 0x00, b: 0x00 },
  Color { r: 0xcc, g: 0x35, b: 0x00 },
  Color { r: 0x5e, g: 0xc8, b: 0x09 },
  Color { r: 0x1d, g: 0x28, b: 0x6f },
  Color { r: 0x00, g: 0xc4, b: 0xff },
  Color { r: 0x8e, g: 0x8e, b: 0x8e },
  Color { r: 0xff, g: 0xe0, b: 0x52 },
  Color { r: 0xff, g: 0xff, b: 0xff },
];

pub const PALETTE_MONOCHROME: [Color; 2] = [Color { r: 0x00, g: 0x00, b: 0x00 }, Color { r: 0xff, g: 0xff, b: 0xff }];
