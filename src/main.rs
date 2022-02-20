mod bmp;
use crate::bmp::BMP;

fn main() {
  let file = BMP::new_from_file("src/images/example.bmp");
  println!("{}", file.get_size(true));
}