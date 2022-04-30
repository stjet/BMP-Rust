mod bmp;
use crate::bmp::BMP;

fn main() {
  let file = BMP::new_from_file("src/images/example.bmp");
  //tests of bmp lib
  let file_size = file.get_size(true);
  println!("File size (bytes): {}", file_size);
  assert_eq!(file_size/1024, BMP::num_bytes_to_kilobytes(file_size));
  assert_eq!((((5 as i8)-(13 as i8)).abs() as u8), 8);
  let dib_header = file.get_dib_header();
  //height, width, bitcount, etc dib size
  if let Ok(unwrapped_dib_header) = dib_header {
    println!("Bitcount (bits per pixel): {}", unwrapped_dib_header.bitcount);
    println!("Height: {} pixels Width: {} pixels", unwrapped_dib_header.height, unwrapped_dib_header.width);
  }
  //println!("{:?}", file.get_color_of_px(0, 0).unwrap());
  //test color functions
  /*let pixel_data = file.get_pixel_data();
  if let Ok(unwrapped_pixel_data) = pixel_data {
    //
  }*/
  println!("Smaller file opened");
  let small_file = BMP::new_from_file("src/images/small_example.bmp");
  //these are currently brg instead of rgb
  println!("{:?}", small_file.get_color_of_px(10, 10).unwrap());
  println!("{:?}", small_file.get_color_of_px(10, 40).unwrap());
  println!("{:?}", small_file.get_color_of_px(40, 10).unwrap());
  println!("{:?}", small_file.get_color_of_px(40, 40).unwrap());
  //let ttt = u32::from_le_bytes([255, 255, 255, 255]);
  //println!("{}", ttt);
  //println!("{}", ttt & 0b00000000111111110000000000000000);
}