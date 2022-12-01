#[path = "../src/lib.rs"]
mod bmp;
use bmp::bmp::BMP;
use std::process::Command;

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
  fn create_blank_bmp(&mut self) {}
  fn add_frame(&mut self, frame: BMP) {
    self.frames.push(frame);
  }
  fn add_empty_frame(&mut self) {
    self.frames.push(BMP::new(self.width, self.height, [0, 0, 0, 0]));
  }
}

//video with export with ffmpeg
pub struct VideoConstructor {
  pub layers: Vec<VideoLayer>,
}

impl VideoConstructor {
  fn new(width: u16, height: u16, layer_num: u8, fps: u8) {
    let layers_vec: Vec<VideoLayer> = vec![];
    for i in 0..layer_num {
      layers_vec.push(VideoLayer::new(width, height));
    }
    return VideoConstructor {
      layers: layers_vec,
    };
  }
  fn export(&self) {
    //loop through self.layers, composite layers frames
    for frame_num in self.layers[0].frames.len() {
      let frame = BMP::new(self.width, self.height, [0, 0, 0, 0]);
      for layer in self.layers {
        frame.draw_image(0, 0, layer.frames[frame_num]);
      }
      //
    }
    //use ffmpeg to convert to video?
    //Command::new("ffmpeg").args([])
    //maybe opencv
  }
}
