use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct McImgArgs {
  /// Path of image to convert
  pub input: String,

  /// Path of image to output
  #[arg(short, long, default_value_t = String::from("./mcimg.png"))]
  pub output: String,

  /// Width of output image, in blocks (1 block = 16*16 pixels)
  #[arg(short, long, default_value_t = 100)]
  pub width: u32,

  /// Path of textures directory
  #[arg(short, long, default_value_t = String::from("./textures"))]
  pub textures: String,

  /// Path of file of blacklisted blocks
  /// Use `-` to ignore (default)
  #[arg(short, long, default_value_t = String::from("-"))]
  pub blacklist: String,

  /// Path of file of required materials
  /// Use `-` to ignore (default)
  #[arg(short, long, default_value_t = String::from("-"))]
  pub materials: String,
}
