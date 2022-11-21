use std::process::Command;

//layer
pub struct VideoLayer {
  fn new(width: u16, height: u16) {
    //
  }
}

impl VideoLayer {
  //
}

//video with export with ffmpeg
pub struct VideoConstructor {
  //
}

impl VideoConstructor {
  fn new(width: u16, height: u16, layer_num: u8, fps: u8) {
    //
  }
  //
  fn export(&self) {
    //use ffmpeg to convert to video?
    //Command::new("ffmpeg").args([])
  }
}
