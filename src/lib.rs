use std::{collections::HashMap, fs, path::PathBuf};

use image::{ImageBuffer, Rgb, Rgba};

/// Resolution of minecraft block (default: 16)
const BLOCK_RES: u32 = 16;

/// Alias for `ImageBuffer` with `Rgb<u8>`
///
/// Only image type used
pub type Image = ImageBuffer<Rgb<u8>, Vec<u8>>;
/// Alias for `HashMap` of `String` to `Rgb<u8>` and `Image`
pub type Map = HashMap<String, (Rgb<u8>, Image)>;

/// Create image from input image and map of blocks
/// 
/// Returns final image and `HashMap` of required blocks
///
/// `original`: `ImageBuffer` of `Rgb<u8>`
///
/// `map`: Map of block images
///
/// `resize`: Width of final image, in pixels
///
/// `filter`: Filter type used for resizing the image
pub fn make_img(
  original: Image,
  map: Map,
  width: u32,
  filter: image::imageops::FilterType,
) -> (Image, HashMap<String, u32>) {
  // Resize image
  let img_resize = image::imageops::resize(
    &original,
    width,
    width * original.height() / original.width(),
    filter,
  );

  // Map pixel to block
  let mut blocks: Vec<String> = vec![];
  for pixel in img_resize.pixels() {
    let block = best_match(pixel, &map);
    blocks.push(block.expect("No best match found for pixel"));
  }

  // Create list of blocks required
  let mut materials: HashMap<String, u32> = HashMap::new();
  for block in &blocks {
    let entry = materials.entry(block.clone()).or_insert(0);
    *entry += 1;
  }

  // Create final image
  let mut img_final = image::ImageBuffer::<Rgb<u8>, Vec<u8>>::new(
    img_resize.width() * BLOCK_RES,
    img_resize.height() * BLOCK_RES,
  );

  // Overlay block images onto final image
  for (i, block) in blocks.iter().enumerate() {
    // Get whole block image from map
    let (_, top) = map.get(block).expect("Unknown block");

    // Find coordinates to place block image
    let x = i % img_resize.width() as usize * BLOCK_RES as usize;
    let y = i / img_resize.width() as usize * BLOCK_RES as usize;

    // Overlay block image
    image::imageops::overlay(&mut img_final, top, x as i64, y as i64);
  }

  (img_final, materials)
}

/// Get map of blocks from file path
/// TODO ? Cache ?
pub fn get_map(path: &str) -> Map {
  let mut map: Map = HashMap::new();

  // Loop files in directory
  for file in fs::read_dir(path)
    .expect("Could not read texture directory")
    .flatten()
  {
    // Get name from filename, continue if invalid
    let name = file.file_name();
    let name = match name.to_str() {
      None => continue,
      Some(x) => x,
    };

    // Continue if file is not png
    if !name.ends_with(".png") {
      continue;
    }

    // Remove file extension, continue if invalid
    let name = match remove_filename_ext(name) {
      None => continue,
      Some(x) => x,
    };

    // Read image if is valid
    if let Some((av, img)) = read_img(file.path()) {
      map.insert(name.to_string(), (av, img));
    }
  }

  map
}

/// Match color with closest color in map
///
/// Returns `None` if map is empty
fn best_match(color: &Rgb<u8>, map: &Map) -> Option<String> {
  // Running best value
  let mut best: Option<(&str, &Rgb<u8>, u32)> = None;

  // Loop block map
  for (name, (map_color, _)) in map {
    // Calculate average difference
    let diff = color_diff(map_color, color);

    // Change best value if difference is less
    if best.is_none() || diff < best.unwrap().2 {
      best = Some((name, map_color, diff));
    }
  }

  best.map(|x| x.0.to_string())
}

/// Get absolute difference between 2 `Rgb` colors
fn color_diff(a: &Rgb<u8>, b: &Rgb<u8>) -> u32 {
  // Sum differences as average
  // No need for division by 3 in this case
  color_comp_diff(a.0[0], b.0[0])
    + color_comp_diff(a.0[1], b.0[1])
    + color_comp_diff(a.0[2], b.0[2])
}

/// Get absolute difference between 2 color components
fn color_comp_diff(a: u8, b: u8) -> u32 {
  (a as i32 - b as i32).abs() as u32
}

/// Returns every character before first `.` in filename
fn remove_filename_ext<'a>(file: &'a str) -> Option<&'a str> {
  let mut split = file.split(".");
  split.next()
}

/// Get average `Rgb` value in `ImageBuffer`
fn img_average(img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Option<Rgb<u8>> {
  // Accumulator of average values
  let mut av_acc = [0usize; 3];

  // Loop pixels
  for rgb in img.pixels() {
    // Return `None` if pixel is not opaque
    if rgb.0[3] < 255 {
      return None;
    }

    // Add rgb values to accumulator
    av_acc[0] += rgb.0[0] as usize;
    av_acc[1] += rgb.0[1] as usize;
    av_acc[2] += rgb.0[2] as usize;
  }

  // Divide rgb values to find average
  let count = img.pixels().len();
  Some(Rgb::from([
    (av_acc[0] / count) as u8,
    (av_acc[1] / count) as u8,
    (av_acc[2] / count) as u8,
  ]))
}

/// Read image from file path as average `Rgb<u8>` color and `ImageBuffer` of `Rgb<u8>`
fn read_img(path: PathBuf) -> Option<(Rgb<u8>, ImageBuffer<Rgb<u8>, Vec<u8>>)> {
  // Open image
  let img = image::open(path)
    .expect("Could not read image file")
    .into_rgba8();

  // Return `None` if image is not standard size
  if img.width() != BLOCK_RES || img.height() != BLOCK_RES {
    return None;
  }

  // Get average color value of image
  let average_color = match img_average(&img) {
    Some(x) => x,
    // Return `None` if image is not entirely opaque
    None => return None,
  };

  Some((average_color, rgba8_to_rgb8(img)))
}

/// Convert `Rgba` image buffer to `Rgb`
fn rgba8_to_rgb8(
  input: image::ImageBuffer<Rgba<u8>, Vec<u8>>,
) -> image::ImageBuffer<Rgb<u8>, Vec<u8>> {
  let width = input.width() as usize;
  let height = input.height() as usize;

  // Get the raw image data as a vector
  let input: &Vec<u8> = input.as_raw();

  // Allocate a new buffer for the RGB image, 3 bytes per pixel
  let mut output_data = vec![0u8; width * height * 3];

  let mut i = 0;
  // Iterate through 4-byte chunks of the image data (RGBA bytes)
  for chunk in input.chunks(4) {
    // ... and copy each of them to output, leaving out the A byte
    output_data[i..i + 3].copy_from_slice(&chunk[0..3]);
    i += 3;
  }

  // Construct a new image
  image::ImageBuffer::from_raw(width as u32, height as u32, output_data).unwrap()
}
