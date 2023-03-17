use std::time::Instant;
#[path = "../src/lib.rs"]
mod bmp;
use bmp::bmp::BMP;

//see time savings by 

fn main() {
  let now = Instant::now();
  {
    let mut file = BMP::new_from_file("example/images/example.bmp");
    file.invert(None).expect("Failed to invert");
    file.save_to_new("example/images/example_invert.bmp").expect("Failed to write to file");
  }
  let elapsed = now.elapsed();
  println!("Elapsed for invert: {:.2?}", elapsed);

  let now = Instant::now();
  {
    let mut file = BMP::new_from_file("example/images/example.bmp");
    file.gaussian_blur(3).expect("Failed to blur");
    file.save_to_new("example/images/example_blur.bmp").expect("Failed to write to file");
  }
  let elapsed = now.elapsed();
  println!("Elapsed for gaussian blur: {:.2?}", elapsed);

  let mut large_scratch_file;
  let now = Instant::now();
  {
    large_scratch_file = BMP::new(500, 500, None);
  }
  let elapsed = now.elapsed();
  println!("Elapsed for large file creation: {:.2?}", elapsed);

  let mut large_scratch_file2;
  let now = Instant::now();
  {
    large_scratch_file2 = large_scratch_file.clone();
  }
  let elapsed = now.elapsed();
  println!("Elapsed for large file cloning: {:.2?}", elapsed);

  let now = Instant::now();
  {
    large_scratch_file.draw_rectangle(Some([128, 128, 128, 255]), Some([128, 128, 128, 255]), [0, 0], [499, 499]).expect("Failed to rect");
    large_scratch_file.save_to_new("example/images/large_128_rect.bmp").expect("Failed to write to file");
  }
  let elapsed = now.elapsed();
  println!("Elapsed for large file fill and stroke rect: {:.2?}", elapsed);

  let now = Instant::now();
  {
    //60 seconds currently...
    large_scratch_file2.fill_bucket([128, 128, 128, 255], 249, 249).expect("Failed to fill bucket");
    large_scratch_file2.save_to_new("example/images/large_128_fill.bmp").expect("Failed to write to file");
  }
  let elapsed = now.elapsed();
  println!("Elapsed for large file fill bucket: {:.2?}", elapsed);
}
