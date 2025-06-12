use clap::Parser;
use dither_lib::args::Args;
use dither_lib::dither;

fn main() {
  // get cli arguments
  let args = Args::parse();
  //dbg!(args);

  // open image
  let (mut buffer, width, height) = dither::open_image(&args.in_img);

  // process image
  dither::dither(&mut buffer, args.dither_type, args.color_palette, width, height);

  // save file
  if let Some(out_img) = args.out_img {
    println!("Saving output image to: {:?}", out_img);
    dither::save_image(buffer, out_img, width, height);
  } else {
    // if no output image is specified, save to the same path with "_out" suffix
    let mut out_path = args.in_img.clone();
    out_path.set_file_name(format!(
      "{}_out.{}",
      out_path.file_stem().unwrap().to_str().unwrap(),
      out_path.extension().unwrap().to_str().unwrap()
    ));
    println!("Saving output image to: {:?}", out_path);
    dither::save_image(buffer, out_path, width, height);
  }
}
