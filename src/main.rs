use std::fs;

use image::imageops::FilterType;

use mcimg::{get_map, make_img, Image, Map};

const TEXTURES_PATH: &str = "./textures/";
const BLACKLIST_PATH: &str = "./blacklist.txt";
const IMG_IN: &str = "./test.jpg";

fn main() {
  let blacklist = fs::read_to_string(BLACKLIST_PATH).expect("Could not read blacklist file");

  let map: Map = get_map(TEXTURES_PATH, &blacklist.lines().collect::<Vec<_>>());

  let original: Image = image::open(IMG_IN)
    .expect("Could not read image file")
    .into_rgb8();

  let (img, materials) = make_img(original, map, 20, FilterType::Nearest);

  img
    .save("./output.png")
    .expect("Could not save final image");

  fs::write(
    "./materials.txt",
    materials
      .iter()
      .map(|(k, v)| format!("{k} : {v}"))
      .collect::<Vec<String>>()
      .join("\n"),
  )
  .expect("Could not save list of materials");

  println!("Success!");
}
