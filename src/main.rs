use std::fs;
use std::convert::TryInto;

struct BMP {
  contents: Vec<u8>,
}

impl BMP {
  fn new(file_path: Option<&str>) -> BMP {
    if file_path != None {
      let contents = fs::read(file_path.unwrap())
        .expect("Error encountered");
      return BMP { contents: contents };
    }
    return BMP { contents: Vec::new() };
  }
  fn get_header(&self) -> &[u8; 14] {
    //turn slice into array
    self.contents[..14].try_into().unwrap()
    //BM, file size, reserved, reserved, offset
    //get file size with i32::from_ne_bytes(<BMP>.get_header()[2..6].try_into().unwrap()) or BMP.contents.len()
    //get offset with i32::from_ne_bytes(file.get_header()[10..14].try_into().unwrap())
  }
}

fn main() {
  let file = BMP::new(Some("src/images/example.bmp"));
  println!("{}", file.contents.len());
  println!("{}", i32::from_ne_bytes(file.get_header()[10..14].try_into().unwrap()));
}