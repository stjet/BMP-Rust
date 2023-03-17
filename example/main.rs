#[path = "../src/lib.rs"]
mod bmp;
use bmp::bmp::BMP;

fn main() {
  println!("{:?}", BMP::composite_colors([233,71,255,200], [0,0,0,0]));
  let file = BMP::new_from_file("example/images/example.bmp");  //tests of bmp lib
  assert_eq!(true, file.is_from_file());
  let file_size = file.get_size(true);
  println!("File size (bytes): {}", file_size);
  assert_eq!(file_size/1024, BMP::num_bytes_to_kilobytes(file_size));
  assert_eq!(BMP::rgba_to_hex([45, 77, 0, 255]), "2D4D00FF".to_string());
  assert_eq!(BMP::hex_to_rgba("F7BA9E11".to_string()), [247, 186, 158, 17]);
  let dib_header = file.get_dib_header();
  //height, width, bitcount, etc dib size
  if let Ok(unwrapped_dib_header) = dib_header {
    println!("Dib header size: {}", unwrapped_dib_header.size);
    println!("Bitcount (bits per pixel): {}", unwrapped_dib_header.bitcount);
    println!("Height: {} pixels Width: {} pixels", unwrapped_dib_header.height, unwrapped_dib_header.width);
    println!("{}", unwrapped_dib_header.clone().height);
  }
  println!("Smaller file opened");
  //pixel data seems to start from bottom left
  let mut small_file = BMP::new_from_file("example/images/small_example.bmp");

  /*
  if let Ok(unwrapped_dib_header2) = small_file.get_dib_header() {
    println!("Gamma: {} {} {}", unwrapped_dib_header2.GammaRed.unwrap(), unwrapped_dib_header2.GammaGreen.unwrap(), unwrapped_dib_header2.GammaBlue.unwrap());
    println!("{:?}", unwrapped_dib_header2.Endpoints.unwrap());
  }
  */
  
  //these are currently brg instead of rgb
  println!("{:?}", small_file.get_color_of_px(10, 10).unwrap());
  println!("{:?}", small_file.get_color_of_px(40, 10).unwrap());
  println!("{:?}", small_file.get_color_of_px(10, 40).unwrap());
  assert_eq!([233, 71, 255, 255],  small_file.get_color_of_px(40, 40).unwrap());
  println!("{}", small_file.get_format());

  //let ttt = u32::from_le_bytes([255, 255, 255, 255]);
  //println!("{}", ttt);
  //println!("{}", ttt & 0b00000000111111110000000000000000);\
  //test color changing
  //y starts at 1 x starts at 0
  small_file.change_color_of_pixel(10, 10, [233, 71, 255, 255]).expect("Failed to change color of pixel");
  //alpha test
  small_file.change_color_of_pixel(15, 15, [255, 126, 16, 0]).expect("Failed to change color of pixel");
  //batch color change
  small_file.change_color_of_pixels(vec![[3,3],[3,4],[4,3],[4,4]], [50, 24, 69, 255]).expect("Failed to change colors of pixels");
  small_file.save_to_new("example/images/e.bmp").expect("Failed to write to file");
  let mut small_file2 = BMP::new_from_file("example/images/e.bmp");
  //fill color may not be right
  //small_file2.fill_bucket([155, 42, 66, 255], 35, 40).expect("Failed to bucket fill");
  small_file2.fill_bucket([155, 42, 66, 255], 1, 1).expect("Failed to bucket fill");
  small_file2.save_to_new("example/images/e2.bmp").expect("Failed to write to file");
  println!("Draw line test");
  //bug: coords are WRONG
  let mut small_file3 = BMP::new_from_file("example/images/small_example.bmp");
  small_file3.draw_line([233, 30, 99, 255], [5, 3], [5, 29]).expect("Failed to draw line");
  small_file3.draw_line([233, 30, 99, 255], [15, 40], [5, 40]).expect("Failed to draw line");
  small_file3.draw_line([233, 30, 99, 255], [1, 2], [40, 2]).expect("Failed to draw line");
  small_file3.save_to_new("example/images/line_test1.bmp").expect("Failed to write to file");
  let mut small_file4 = BMP::new_from_file("example/images/small_example.bmp");
  small_file4.draw_line([233, 30, 99, 255], [15, 32], [8, 20]).expect("Failed to draw line"); //backwards test
  small_file4.draw_line([233, 30, 99, 255], [3, 3], [14, 12]).expect("Failed to draw line");
  small_file4.draw_line([100, 65, 45, 255], [20, 20], [52, 52]).expect("Failed to draw line");
  small_file4.save_to_new("example/images/line_test2.bmp").expect("Failed to write to file");
  let mut small_file5 = BMP::new_from_file("example/images/small_example.bmp");
  small_file5.draw_line([125, 125, 170, 255], [4, 7], [8, 9]).expect("Failed to draw line");
  small_file5.draw_line([255, 255, 255, 255], [8, 25], [40, 29]).expect("Failed to draw line");
  small_file5.draw_line([0, 255, 0, 255], [8, 43], [40, 44]).expect("Failed to draw line");
  small_file5.draw_line([0, 120, 11, 255], [35, 2], [36, 12]).expect("Failed to draw line");
  small_file5.draw_line([100, 65, 45, 255], [4, 20], [7, 37]).expect("Failed to draw line");
  small_file5.save_to_new("example/images/line_test3.bmp").expect("Failed to write to file");
  //IS THIS LOSS.JPG? NOPE ITS LOSS.BMP
  let mut small_file6 = BMP::new_from_file("example/images/small_example.bmp");
  small_file6.draw_line([255, 255, 255, 255], [10, 5], [10, 20]).expect("Failed to draw line");
  small_file6.draw_line([255, 255, 255, 255], [30, 5], [30, 20]).expect("Failed to draw line");
  small_file6.draw_line([255, 255, 255, 255], [40, 10], [40, 20]).expect("Failed to draw line");
  small_file6.draw_line([255, 255, 255, 255], [10, 35], [10, 45]).expect("Failed to draw line");
  small_file6.draw_line([255, 255, 255, 255], [20, 35], [20, 45]).expect("Failed to draw line");
  small_file6.draw_line([255, 255, 255, 255], [40, 35], [40, 45]).expect("Failed to draw line");
  small_file6.draw_line([255, 255, 255, 255], [35, 42], [50, 42]).expect("Failed to draw line");
  small_file6.save_to_new("example/images/loss.bmp").expect("Failed to write to file");
  println!("Draw rect test");
  let mut small_file7 = BMP::new_from_file("example/images/small_example.bmp");
  small_file7.draw_rectangle(None, Some([255, 255, 255, 255]), [0,2], [15,11]).expect("Failed to draw rect");
  //problems here, outline not drawn correctly
  small_file7.draw_rectangle(Some([0, 0, 0, 255]), Some([255, 255, 255, 255]), [40, 7], [45, 19]).expect("Failed to draw rect");
  small_file7.save_to_new("example/images/rect_test.bmp").expect("Failed to write to file");
  //invert test
  println!("Invert test");
  let mut small_file8 = BMP::new_from_file("example/images/small_example.bmp");
  small_file8.invert(None).expect("Failed to invert");
  small_file8.save_to_new("example/images/invert_test.bmp").expect("Failed to write to file");
  //circle test
  println!("Ellipse test");
  let mut small_file9 = BMP::new_from_file("example/images/small_example.bmp");
  small_file9.draw_ellipse([23, 25], 10, 12, [255, 0, 0, 255], Some([125, 64, 64, 255]), true).expect("Failed to draw ellipse");
  small_file9.draw_ellipse([8, 8], 4, 4, [255, 0, 0, 255], None, false).expect("Failed to draw ellipse");
  //small_file9.draw_ellipse([13, 35], 4, 5, [255, 0, 0, 255], None);
  small_file9.save_to_new("example/images/ellipse_test.bmp").expect("Failed to write to file");
  //opacity change
  println!("Opacity test");
  let mut small_file10 = BMP::new_from_file("example/images/small_example.bmp");
  small_file10.change_opacity(15).expect("Failed to change opacity");
  if small_file10 == small_file10.clone() {
    println!("clone success");
  }
  small_file10.save_to_new("example/images/opacity_test.bmp").expect("Failed to write to file");
    let mut small_file11 = BMP::new_from_file("example/images/small_example.bmp");
  small_file11.change_opacity(200).expect("Failed to change opacity");
  small_file11.save_to_new("example/images/opacity_test2.bmp").expect("Failed to write to file");
  //new file test
  println!("New file test");
  let mut new_file = BMP::new(125, 125, None);
  let new_file_header = new_file.get_header();
  assert_eq!(138, new_file_header.bfOffBits);
  println!("Draw image on another test");
  new_file.draw_image(12, 19, BMP::new_from_file("example/images/small_example.bmp")).expect("Failed to draw image");
  new_file.save_to_new("example/images/artificial.bmp").expect("Failed to write to file");
  let new_file2 = BMP::new(15, 15, None);
  let new_file3 = BMP::new(15, 15, None);
  println!("{}", new_file2 == new_file3);
  //new from scratch default color test
  println!("New file default color test");
  let mut scratch_custom = BMP::new(125, 125, Some([128, 64, 128, 200]));
  //alpha compositing test
  println!("Draw image alpha compositing test");
  scratch_custom.draw_image(20, 20, BMP::new_from_file("example/images/opacity_test.bmp")).expect("Failed to draw image");
  scratch_custom.save_to_new("example/images/scratch_custom.bmp").expect("Failed to write to file");
  //translate test
  println!("Translate test");
  let mut translate_file = BMP::new_from_file("example/images/opacity_test.bmp");
  let mut translate_file2 = translate_file.clone();
  let mut translate_file3 = translate_file.clone();
  translate_file.translate(-3, 5).expect("Translate failed");
  translate_file.save_to_new("example/images/translate1.bmp").expect("Failed to write to file");
  translate_file2.translate(0, 2).expect("Translate failed");
  translate_file2.save_to_new("example/images/translate2.bmp").expect("Failed to write to file");
  translate_file3.translate(-25, 40).expect("Translate failed");
  translate_file3.save_to_new("example/images/translate3.bmp").expect("Failed to write to file");
  println!("Rotate test");
  let mut rotate_file = BMP::new_from_file("example/images/small_example.bmp");
  let mut rotate_file2 = rotate_file.clone();
  let mut rotate_file3 = rotate_file.clone();
  rotate_file2.rotate(-39.0).expect("Rotate failed");
  rotate_file2.save_to_new("example/images/rotate2.bmp").expect("Failed to write to file");
  rotate_file3.rotate(275.0).expect("Rotate failed");
  rotate_file3.save_to_new("example/images/rotate3.bmp").expect("Failed to write to file");
  let mut blur_file = BMP::new_from_file("example/images/blur_attempt.bmp");
  blur_file.gaussian_blur(3).expect("Failed to blur");
  blur_file.save_to_new("example/images/gaussian_blur.bmp").expect("Failed to write to file");
  let mut blur_file2 = BMP::new_from_file("example/images/blur_attempt2.bmp");
  blur_file2.gaussian_blur(4).expect("Failed to blur");
  blur_file2.save_to_new("example/images/gaussian_blur2.bmp").expect("Failed to write to file");
  println!("Grayscale test");
  let mut grayscale_file1 = BMP::new_from_file("example/images/small_example.bmp");
  grayscale_file1.greyscale().expect("Failed to grayscale");
  grayscale_file1.save_to_new("example/images/grayscale.bmp").expect("Failed to write to file");
  let mut grayscale_file2 = BMP::new_from_file("example/images/small_example.bmp");
  grayscale_file2.channel_grayscale(bmp::bmp::RGBAChannel::Red).expect("Failed to channel grayscale");
  grayscale_file2.save_to_new("example/images/grayscale_r.bmp").expect("Failed to write to file");
  let mut grayscale_file3 = BMP::new_from_file("example/images/small_example.bmp");
  grayscale_file3.channel_grayscale(bmp::bmp::RGBAChannel::Green).expect("Failed to channel grayscale");
  grayscale_file3.save_to_new("example/images/grayscale_g.bmp").expect("Failed to write to file");
  let mut grayscale_file4 = BMP::new_from_file("example/images/small_example.bmp");
  grayscale_file4.channel_grayscale(bmp::bmp::RGBAChannel::Blue).expect("Failed to channel grayscale");
  grayscale_file4.save_to_new("example/images/grayscale_b.bmp").expect("Failed to write to file");
  let mut grayscale_file5 = BMP::new_from_file("example/images/small_example.bmp");
  grayscale_file5.channel_grayscale(bmp::bmp::RGBAChannel::Alpha).expect("Failed to channel grayscale");
  grayscale_file5.save_to_new("example/images/grayscale_a.bmp").expect("Failed to write to file");
  //test diff
  //default color is [255, 255, 255, 255]
  println!("Diff test");
  let tiny_file1 = BMP::new(2, 2, None);
  let tiny_file2 = BMP::new(2, 3, None);
  let diff_1_2 = BMP::diff(&tiny_file1, &tiny_file2).unwrap();
  assert_eq!(diff_1_2[0].color1, None);
  assert_eq!(diff_1_2[0].color2, Some([255, 255, 255, 255]));
  let tiny_file3 = BMP::new(2, 3, Some([100, 100, 23, 255]));
  let diff_1_3 = BMP::diff(&tiny_file1, &tiny_file3).unwrap();
  assert_eq!(diff_1_3[0].color1, Some([255, 255, 255, 255]));
  assert_eq!(diff_1_3[0].color2, Some([100, 100, 23, 255]));
  let tiny_file4 = BMP::new(2, 2, Some([78, 222, 12, 150]));
  let diff_1_4 = BMP::diff(&tiny_file1, &tiny_file4).unwrap();
  assert_eq!(diff_1_4.is_same_size(), true);
}
