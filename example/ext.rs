#[path = "../src/lib.rs"]
mod bmp;
use bmp::bmp::BMP;
use std::process::Command;
use opencv::{core, prelude::*, imgcodecs};

//layer
pub struct VideoLayer {
  width: u16,
  height: u16,
  pub frames: Vec<BMP>,
}

impl VideoLayer {
  fn new(width: u16, height: u16) {
    return VideoLayer{
      width: width,
      height: height,
      frames: vec![],
    };
  }
  fn create_blank_bmp(&mut self) {
    return BMP::new(self.width, self.height, [0, 0, 0, 0])
  }
  fn add_frame(&mut self, frame: BMP) {
    self.frames.push(frame);
  }
  fn add_empty_frame(&mut self) {
    self.frames.push(self.create_blank_bmp());
  }
}

//video with export with ffmpeg
pub struct VideoConstructor {
  pub layers: Vec<VideoLayer>,
}

impl VideoConstructor {
  fn new(width: u16, height: u16, layer_num: u8, fps: f64) {
    let layers_vec: Vec<VideoLayer> = vec![];
    for i in 0..layer_num {
      layers_vec.push(VideoLayer::new(width, height));
    }
    return VideoConstructor {
      layers: layers_vec,
    };
  }
  fn export(&self, filename: &str) {
    let opencv_frames_vec: Vec<Mat> = vec![];
    //loop through self.layers, composite layers frames
    for frame_num in self.layers[0].frames.len() {
      let frame = BMP::new(self.width, self.height, [0, 0, 0, 0]);
      for layer in self.layers {
        frame.draw_image(0, 0, layer.frames[frame_num]);
      }
      //turn bmp into ToInputArray, probably?
      //https://github.com/twistedfall/opencv-rust/blob/master/tests/vector.rs
      //or write file, read as opencv image?
      frame.save_to_new("frame.bmp").expect("Failed to write frame to file");
      opencv_frames_vec.push(imgcodecs::imread("frame.bmp", imgcodecs::IMREAD_COLOR).unwrap());
    }
    //use ffmpeg to convert to video?
    //Command::new("ffmpeg").args([])
    //maybe opencv https://github.com/twistedfall/opencv-rust
    //https://docs.rs/opencv/latest/opencv/videoio/struct.VideoWriter.html
    let writer = opencv::videoio::VideoWriter::new(filename, VideoWriter::fourcc('a', 'v', 'c', '1').unwrap(), fps, Size::new(self.width, self.height), true);
    for opencv_frame in opencv_frames_vec {
      //writer.write(image: &dyn ToInputArray)?
      writer.write(opencv_frame)?;
    }
    //
  }
}
