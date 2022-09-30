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
  /*
  let pixel_data = file.get_pixel_data();
  if let Ok(unwrapped_pixel_data) = pixel_data {
  }
  */
  println!("Smaller file opened");
  //pixel data seems to start from bottom left
  let mut small_file = BMP::new_from_file("src/images/small_example.bmp");
  //these are currently brg instead of rgb
  println!("{:?}", small_file.get_color_of_px(10, 10).unwrap());
  println!("{:?}", small_file.get_color_of_px(40, 10).unwrap());
  println!("{:?}", small_file.get_color_of_px(10, 40).unwrap());
  println!("{:?}", small_file.get_color_of_px(40, 40).unwrap());
  //let ttt = u32::from_le_bytes([255, 255, 255, 255]);
  //println!("{}", ttt);
  //println!("{}", ttt & 0b00000000111111110000000000000000);\
  //test color changing
  //y starts at 1 x starts at 0
  small_file.change_color_of_pixel(10, 10, [233, 71, 255, 255]);
  //alpha test
  small_file.change_color_of_pixel(15, 15, [255, 126, 16, 0]);
  small_file.save_to_new("src/images/e.bmp");
  let mut small_file2 = BMP::new_from_file("src/images/e.bmp");
  //fill color may not be right
  small_file2.fill_bucket([155, 42, 66, 255], 35, 40);
  small_file2.save_to_new("src/images/e2.bmp");
  println!("Draw line test");
  //bug: coords are WRONG
  let mut small_file3 = BMP::new_from_file("src/images/small_example.bmp");
  small_file3.draw_line([233, 30, 99, 255], [5, 3], [5, 29]);
  small_file3.draw_line([233, 30, 99, 255], [15, 40], [5, 40]);
  small_file3.draw_line([233, 30, 99, 255], [1, 2], [40, 2]);
  small_file3.save_to_new("src/images/line_test1.bmp");
  let mut small_file4 = BMP::new_from_file("src/images/small_example.bmp");
  small_file4.draw_line([233, 30, 99, 255], [15, 32], [8, 20]); //backwards test
  small_file4.draw_line([233, 30, 99, 255], [3, 3], [14, 12]);
  small_file4.draw_line([100, 65, 45, 255], [20, 20], [52, 52]);
  small_file4.save_to_new("src/images/line_test2.bmp");
  let mut small_file5 = BMP::new_from_file("src/images/small_example.bmp");
  small_file5.draw_line([125, 125, 170, 255], [4, 7], [8, 9]);
  small_file5.draw_line([255, 255, 255, 255], [8, 25], [40, 29]);
  small_file5.draw_line([0, 255, 0, 255], [8, 43], [40, 44]);
  small_file5.draw_line([0, 120, 11, 255], [35, 2], [36, 12]);
  small_file5.draw_line([100, 65, 45, 255], [4, 20], [7, 37]);
  small_file5.save_to_new("src/images/line_test3.bmp");
  //IS THIS LOSS.JPG? NOPE ITS LOSS.BMP
  let mut small_file6 = BMP::new_from_file("src/images/small_example.bmp");
  small_file6.draw_line([255, 255, 255, 255], [10, 5], [10, 20]);
  small_file6.draw_line([255, 255, 255, 255], [30, 5], [30, 20]);
  small_file6.draw_line([255, 255, 255, 255], [40, 10], [40, 20]);
  small_file6.draw_line([255, 255, 255, 255], [10, 35], [10, 45]);
  small_file6.draw_line([255, 255, 255, 255], [20, 35], [20, 45]);
  small_file6.draw_line([255, 255, 255, 255], [40, 35], [40, 45]);
  small_file6.draw_line([255, 255, 255, 255], [35, 42], [50, 42]);
  small_file6.save_to_new("src/images/loss.bmp");
  println!("Draw rect test");
  let mut small_file7 = BMP::new_from_file("src/images/small_example.bmp");
  small_file7.draw_rectangle(None, Some([255, 255, 255, 255]), [0,2], [15,11]);
  //problems here, outline not drawn correctly
  small_file7.draw_rectangle(Some([0, 0, 0, 255]), Some([255, 255, 255, 255]), [40, 7], [45, 19]);
  small_file7.save_to_new("src/images/rect_test.bmp");
  //invert test
  println!("Invert test");
  let mut small_file8 = BMP::new_from_file("src/images/small_example.bmp");
  small_file8.invert(None);
  small_file8.save_to_new("src/images/invert_test.bmp");
}