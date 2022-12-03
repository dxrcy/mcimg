use std::fs;

use clap::Parser;
use image::imageops::FilterType;

use mcimg::{args::McImgArgs, get_map, make_img, Image, Map};

//TODO ! Better error handling !
fn main() {
  // Parse arguments
  let args = McImgArgs::parse();

  println!("\x1b[1mMcImg\x1b[0m");

  // Get blacklist file, if not `-`
  let blacklist = if args.blacklist != "-" {
    fs::read_to_string(args.blacklist)
      .expect("Could not read blacklist file")
      .lines()
      .map(|x| x.to_string())
      .collect::<Vec<_>>()
  } else {
    vec![]
  };

  // Get map of blocks
  println!("  Mapping textures to average colors...");
  let map: Map = get_map(&args.textures, &blacklist);

  // Open original input image
  println!("  Parsing input image...");
  let original: Image = image::open(args.input)
    .expect("Could not read image file")
    .into_rgb8();

  // Create output image
  println!(
    "  Creating image with width of {} pixels...",
    args.width * mcimg::BLOCK_RES
  );
  //TODO Add filter to args ?
  let (img, materials) = make_img(original, map, args.width, FilterType::Nearest);

  // Save output image
  println!("  Saving image...");
  img.save(args.output).expect("Could not save final image");

  // Create materials file, if not `-`
  if args.materials != "-" {
    fs::write(
      args.materials,
      materials
        .iter()
        .map(|(k, v)| format!("{k} : {v}"))
        .collect::<Vec<String>>()
        .join("\n"),
    )
    .expect("Could not save list of materials");
  }

  println!("\x1b[1mSuccess!\x1b[0m");
}
