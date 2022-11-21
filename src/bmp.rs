use std::fs;
use std::convert::TryInto;
use std::collections::VecDeque;
use std::fmt;
use std::collections::HashMap;
use std::io::Write;
//use std::io::ErrorKind;

//support packed dibs, dibs that have no empty gaps

/*
Documentation - important links
https://docs.microsoft.com/en-us/windows/win32/gdi/bitmap-header-types
https://en.wikipedia.org/wiki/BMP_file_format#File_structure
http://fileformats.archiveteam.org/wiki/BMP
*/

const HEADER_OFFSET: usize = 14;

//Errors
pub enum ErrorKind {
  Unsupported,
  DoesNotExist,
  WrongFileType,
  UseExtraBitMasks,
  FailedToWrite,
  Missing,
}

impl ErrorKind {
  fn as_str(&self) -> &str {
    match *self {
      ErrorKind::Unsupported => "File is unsupported",
      ErrorKind::DoesNotExist => "Requested object does not exist",
      ErrorKind::WrongFileType => "Wrong file type. Must be a .bmp file",
      ErrorKind::UseExtraBitMasks => "Use extra bit masks instead",
      ErrorKind::FailedToWrite => "Failed to write to file",
      ErrorKind::Missing => "Missing expected parameter or object",
    }
  }
}

impl fmt::Debug for ErrorKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self.as_str())
  }
}

impl fmt::Display for ErrorKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Error: {}", self.as_str())
  }
}

//File header
#[allow(non_snake_case)]
pub struct BITMAPFILEHEADER {
  pub bfType: String,
  pub bfSize: u32,
  pub bfReserved1: Vec<u8>,
  pub bfReserved2: Vec<u8>,
  pub bfOffBits: u32,
}

impl IntoIterator for BITMAPFILEHEADER {
  type Item = u8;
  type IntoIter = std::array::IntoIter<u8, 14>;

  fn into_iter(self) -> Self::IntoIter {
    let bytes_vec = [self.bfType.as_bytes(), &self.bfSize.to_le_bytes(), &BMP::vec_to_2u8_array(self.bfReserved1), &BMP::vec_to_2u8_array(self.bfReserved2), &self.bfOffBits.to_le_bytes()].concat();
    let mut bytes_array: [u8; 14] = [0u8; 14];
    //vector.len() should be 14
    for i in 0..bytes_vec.len() {
      bytes_array[i] = bytes_vec[i];
    }
    //DEPRECATED
    //return std::array::IntoIter::new(bytes_array);
    return IntoIterator::into_iter(bytes_array);
  }
}

/*
DIB Headers
struct BITMAPCOREHEADER {
  size: u16,
  width: u32,
  height: u32,
  planes: u16,
  bitcount: u16,
}
if biCompression is BI_ALPHABITFIELDS or BI_BITFIELDS 
struct BITMAPINFOHEADER {
  size: u16,
  width: u32,
  biHeight can be negative
  height: i32,
  planes: u16,
  bitcount: u16,
  compression: String,
  sizeimage: u32,
  XPelsPerMeter: u32,
  YPelsPerMeter: u32,
  ClrUsed: u32,
  ClrImportant: u32,
}
struct BITMAPV4HEADER {
  size: u16,
  width: u32,
  bV4Height can be negative
  height: i32,
  planes: u16,
  bitcount: u16,
  compression: String,
  sizeimage: u32,
  XPelsPerMeter: u32,
  YPelsPerMeter: u32,
  ClrUsed: u32,
  ClrImportant: u32,
  RedMask: u32,
  GreenMask: u32,
  BlueMask: u32,
  AlphaMask: u32,
  CSType: String,
  rgb
  Endpoints: [[i32; 3]; 3],
  GammaRed: u32,
  GammaGreen: u32,
  GammaBlue: u32,
}
struct BITMAPV5HEADER {
  size: u16,
  width: u32,
  height: i32,
  planes: u16,
  bitcount: u16,
  compression: String,
  sizeimage: u32,
  XPelsPerMeter: u32,
  YPelsPerMeter: u32,
  ClrUsed: u32,
  ClrImportant: u32,
  RedMask: u32,
  GreenMask: u32,
  BlueMask: u32,
  AlphaMask: u32,
  CSType: String,
  Endpoints: [[i32; 3]; 3],
  GammaRed: u32,
  GammaGreen: u32,
  GammaBlue: u32,
  Intent: String,
  ProfileData: u16,
  ProfileSize: u16,
  Reserved: Vec<u8>,
}

enum DIBHEADER {
  BITMAPCOREHEADER(BITMAPCOREHEADER),
  BITMAPINFOHEADER(BITMAPINFOHEADER),
  BITMAPV4HEADER(BITMAPV4HEADER),
  BITMAPV5HEADER(BITMAPV5HEADER),
}
*/

//DIB header
#[allow(non_snake_case)]
#[derive(Clone)]
pub struct DIBHEADER {
  pub size: u32,
  pub width: u32,
  pub height: i32,
  pub planes: u16,
  pub bitcount: u16,
  pub compression: Option<String>,
  pub sizeimage: Option<u32>,
  pub XPelsPerMeter: Option<u32>,
  pub YPelsPerMeter: Option<u32>,
  pub ClrUsed: Option<u32>,
  pub ClrImportant: Option<u32>,
  pub RedMask: Option<u32>,
  pub GreenMask: Option<u32>,
  pub BlueMask: Option<u32>,
  pub AlphaMask: Option<u32>,
  pub CSType: Option<String>,
  pub Endpoints: Option<[[i32; 3]; 3]>,
  pub GammaRed: Option<u32>,
  pub GammaGreen: Option<u32>,
  pub GammaBlue: Option<u32>,
  pub Intent: Option<String>,
  pub ProfileData: Option<u16>,
  pub ProfileSize: Option<u16>,
  pub Reserved: Option<Vec<u8>>,
}

//for non 124 byte dib headers, cut off excess bytes
impl IntoIterator for DIBHEADER {
  //DIBHEADER example: line 504ish
  type Item = u8;
  type IntoIter = std::array::IntoIter<u8, 124>;

  fn into_iter(self) -> Self::IntoIter {
    //let bytes_vec = [self.bfType.as_bytes(), &self.bfSize.to_le_bytes(), &BMP::vec_to_2u8_array(self.bfReserved1), &BMP::vec_to_2u8_array(self.bfReserved2), &self.bfOffBits.to_le_bytes()].concat();
    let mut bytes_vec = [&self.size.to_le_bytes()[..], &self.width.to_le_bytes(), &self.height.to_le_bytes(), &self.planes.to_le_bytes(), &self.bitcount.to_le_bytes()].concat();
    if self.size > 12 {
      let compression_table: HashMap<String, u32> = HashMap::from([
        ("BI_RGB".to_string(), 0),
        ("BI_RLE8".to_string(), 1),
        ("BI_RLE4".to_string(), 2),
        ("BI_BITFIELDS".to_string(), 3),
        ("BI_JPEG".to_string(), 4),
        ("BI_PNG".to_string(), 5),
        ("BI_ALPHABITFIELDS".to_string(), 6)
      ]);
      let compression: u32 = *compression_table.get(&self.compression.unwrap()).unwrap();
      bytes_vec.append(&mut [&compression.to_le_bytes()[..], &self.sizeimage.unwrap().to_le_bytes(), &self.XPelsPerMeter.unwrap().to_le_bytes(), &self.YPelsPerMeter.unwrap().to_le_bytes(), &self.ClrUsed.unwrap().to_le_bytes(), &self.ClrImportant.unwrap().to_le_bytes()].concat());
    }
    if self.size > 40 {
      let mut endpoints_l: [u8; 36] = [0u8; 36];
      let endpoints = self.Endpoints.unwrap();
      //create endpoints list
      for i in 0..3 {
        for ii in 0..3 {
          //4 bytes
          endpoints_l[(i*3+ii*4) as usize] = endpoints[i as usize][ii as usize].to_le_bytes()[0];
          endpoints_l[(i*3+ii*4+1) as usize] = endpoints[i as usize][ii as usize].to_le_bytes()[1];
          endpoints_l[(i*3+ii*4+2) as usize] = endpoints[i as usize][ii as usize].to_le_bytes()[2];
          endpoints_l[(i*3+ii*4+3) as usize] = endpoints[i as usize][ii as usize].to_le_bytes()[3];
        }
      }
      //println!("{:?}", endpoints_l);
      //wip
      bytes_vec.append(&mut [&self.RedMask.unwrap().to_le_bytes(), &self.GreenMask.unwrap().to_le_bytes(), &self.BlueMask.unwrap().to_le_bytes(), &self.AlphaMask.unwrap().to_le_bytes(), self.CSType.unwrap().as_bytes(), &endpoints_l[..], &self.GammaRed.unwrap().to_le_bytes(), &self.GammaGreen.unwrap().to_le_bytes(), &self.GammaBlue.unwrap().to_le_bytes()].concat());
    }
    if self.size > 108 {
      //Reserved
      let mut reserved_l: [u8; 4] = [0u8; 4];
      let reserved = self.Reserved.unwrap();
      for i in 0..4 {
        reserved_l[i as usize] = reserved[i as usize];
      }
      //Intent
      let intent_table: HashMap<String, u32> = HashMap::from([        ("LCS_GM_ABS_COLORIMETRIC".to_string(), 0),
        ("LCS_GM_BUSINESS".to_string(), 1),
        ("LCS_WINDOWS_COLOR_SPACE".to_string(), 2),
        ("LCS_GM_GRAPHICS".to_string(), 3),
        ("LCS_GM_IMAGES".to_string(), 4)
      ]);
      let intent: u32 = *intent_table.get(&self.Intent.unwrap()).unwrap();
      bytes_vec.append(&mut [&intent.to_le_bytes()[..], &self.ProfileData.unwrap().to_le_bytes(), &self.ProfileSize.unwrap().to_le_bytes(), &reserved_l[..]].concat());
    }
    let mut bytes_array: [u8; 124] = [0u8; 124];
    //vector.len() should be 124
    for i in 0..bytes_vec.len() {
      bytes_array[i] = bytes_vec[i];
    }
    //DEPRECATED
    //return std::array::IntoIter::new(bytes_array);
    return IntoIterator::into_iter(bytes_array);
  }
}
  
//rgbtriple and rgbquad
pub enum ColorTable {
  RGBTRIPLE(Vec<[u8; 3]>),
  RGBQUAD(Vec<[u8; 4]>),
}

//extra bit masks, these are unofficial names
#[allow(non_camel_case_types)]
pub struct BI_BITFIELDS_MASKS {
  pub red: u32,
  pub green: u32,
  pub blue: u32,
}

#[allow(non_camel_case_types)]
pub struct BI_ALPHABITFIELDS_MASKS {
  pub red: u32,
  pub green: u32,
  pub blue: u32,
  pub alpha: u32,
}

#[allow(non_camel_case_types)]
enum EXTRA_BIT_MASKS {
  BI_BITFIELDS_MASKS(BI_BITFIELDS_MASKS),
  BI_ALPHABITFIELDS_MASKS(BI_ALPHABITFIELDS_MASKS),
}

pub struct BMP {
  pub contents: Vec<u8>,
  from_file: bool,
  //bitmap_file_header: BITMAPFILEHEADER,
  //dib_header: DIBHEADER,
}

impl PartialEq for BMP {
  fn eq(&self, other: &BMP) -> bool {
    self.contents == other.contents
  }
}

impl Clone for BMP {
  fn clone(&self) -> BMP {
    let mut clone_bmp = BMP::new(1, 1, None);
    clone_bmp.contents = self.contents.to_vec();
    return clone_bmp;
  }
}

impl BMP {
  //have a file header, generate (40 bytes? 108 bytes? 124 bytes?) dib header, load in [0, 0, 0, 0] for pixels in pixel data in bgra
  pub fn new(height: i32, width: u32, default_color: Option<[u8; 4]>) -> BMP {
    let mut contents = Vec::new();
    //file header
    let offset: u32 = 14+124;
    let size: u32 = offset as u32+(height.abs() as u32 * width)*4;
    let file_header = BITMAPFILEHEADER {
      bfType: "BM".to_string(),
      bfSize: size,
      bfReserved1: vec![0, 0],
      bfReserved2: vec![0, 0],
      bfOffBits: offset,
    };
    //turns into bytes, and adds it
    contents.extend(file_header);
    //use v5: 124 bytes
    let dib_header = DIBHEADER {
      size: 124,
      width: width,
      height: height,
      planes: 1,
      bitcount: 32,
      compression: Some("BI_RGB".to_string()),
      sizeimage: Some(size),
      //96 dpi
      XPelsPerMeter: Some(3780),
      YPelsPerMeter: Some(3780),
      //no color table, so ClrUsed and ClrImportant are 0
      ClrUsed: Some(0),
      ClrImportant: Some(0),
      //ARGB?
      RedMask: Some(16711680),
      GreenMask: Some(65280),
      BlueMask: Some(255),
      AlphaMask: Some(4278190080),
      //"BGRs" why? I don't know and I will not question
      CSType: Some("BGRs".to_string()),
      //I don't know what these numbers mean :)
      Endpoints: Some([[687194752, 354334816, 32212256], [322122560, 644245120, 107374144], [161061280, 64424508, 848256036]]),
      GammaRed: Some(0),
      GammaGreen: Some(0),
      GammaBlue: Some(0),
      Intent: Some("LCS_GM_IMAGES".to_string()),
      ProfileData: Some(0),
      ProfileSize: Some(0),
      Reserved: Some(vec![0, 0, 0, 0]),
    };
    contents.extend(dib_header);
    //pixels, turn it into the bytes
    //no rounding is needed, because each row is rounded up to a multiple of u32, which all our pixels are
    for _pixel_num in 0..(width*height as u32) {
      //white by default
      if default_color.is_some() {
        contents.extend(default_color.unwrap());
      } else {
        contents.extend([255, 255, 255, 255]);
      }
    }
    //println!("{:?}", contents);
    return BMP { contents: contents, from_file: false };
  }
  pub fn new_from_file(file_path: &str) -> BMP {
    let contents = fs::read(file_path)
      .expect("Error encountered");
    return BMP { contents: contents, from_file: true, };
  }
  //utilities
  fn bytes_to_int(bytes: [u8; 4]) -> u32 {
    u32::from_le_bytes(bytes)
  }
  fn two_bytes_to_int(bytes: [u8; 2]) -> u16 {
    u16::from_le_bytes(bytes)
  }
  fn byte_to_int(byte: u8) -> u8 {
    u8::from_le_bytes([byte])
  }
  fn two_bytes_to_signed_int(bytes: [u8; 2]) -> i16 {
    i16::from_le_bytes(bytes)
  }
  fn bytes_to_signed_int(bytes: [u8; 4]) -> i32 {
    i32::from_le_bytes(bytes)
  }
  fn bytes_to_string(bytes: &[u8]) -> String {
    String::from_utf8_lossy(&bytes).to_string()
  }
  pub fn num_bytes_to_kilobytes(bytes: u32) -> u32 {
    //1024 bytes per kilobyte
    bytes/1024
  }
  fn vec_to_4u8_array(vector: Vec<u8>) -> [u8; 4] {
    let mut array: [u8; 4] = [0u8; 4];
    //vector.len() should be 4
    for i in 0..vector.len() {
      array[i] = vector[i];
    }
    return array;
  }
  fn vec_to_2u8_array(vector: Vec<u8>) -> [u8; 2] {
    let mut array: [u8; 2] = [0u8; 2];
    //vector.len() should be 2
    for i in 0..vector.len() {
      array[i] = vector[i];
    }
    return array;
  }
  fn vec_to_1u8_array(vector: Vec<u8>) -> [u8; 1] {
    let mut array: [u8; 1] = [0u8; 1];
    //vector.len() should be 1
    for i in 0..vector.len() {
      array[i] = vector[i];
    }
    return array;
  }
  fn int_to_compression(int: u32) -> String {
    let compression_table: HashMap<u32, String> = HashMap::from([
      (0, "BI_RGB".to_string()),
      (1, "BI_RLE8".to_string()),
      (2, "BI_RLE4".to_string()),
      (3, "BI_BITFIELDS".to_string()),
      (4, "BI_JPEG".to_string()),
      (5, "BI_PNG".to_string()),
      (6, "BI_ALPHABITFIELDS".to_string())
    ]);
    return compression_table.get(&int).unwrap().to_string();
  }
  fn int_to_intent(int: u32) -> String {
    let intent_table: HashMap<u32, String> = HashMap::from([
      (0, "LCS_GM_ABS_COLORIMETRIC".to_string()),
      (1, "LCS_GM_BUSINESS".to_string()),
      (2, "LCS_WINDOWS_COLOR_SPACE".to_string()),
      (3, "LCS_GM_GRAPHICS".to_string()),
      (4, "LCS_GM_IMAGES".to_string())
    ]);
    return intent_table.get(&int).unwrap().to_string();
  }
  //color related utilities
  fn alpha_to_percentage(alpha: u8) -> f64 {
    //.into() turns the u8 into f64 (expected return type)
    return (alpha as f64/255 as f64).into();
  }
  fn percentage_to_alpha(percentage: f64) -> u8 {
    return (percentage * 255 as f64).round() as u8;
  }
  fn rgb_to_color(rgb: [u8; 3]) -> String {
    //changes rgb to readable color. (takes rgba) eg: black
    //finds closest color and returns
    //colors.keys()
    //([0, 49, 83], "prussian blue")
    let colors: HashMap<[u8; 3], String> = HashMap::from([
      ([255, 255, 255], "white".to_string()),
      ([0, 0, 0], "black".to_string()),
      ([255, 0, 0], "red".to_string()),
      ([0, 255, 0], "green".to_string()),
      ([0, 0, 255], "blue".to_string()),
      ([255, 128, 0], "orange".to_string()),
      ([128, 64, 0], "brown".to_string()),
      ([0, 128, 0], "dark green".to_string()),
      ([255, 255, 0], "yellow".to_string()),
      ([128, 128, 128], "gray".to_string()),
      ([255, 192, 203], "pink".to_string()),
      ([128, 0, 128], "purple".to_string()),
      ([0, 128, 255], "azure".to_string()),
      ([183, 65, 14], "rust".to_string()),
      ([0, 128, 128], "teal".to_string()),
      ([192, 192, 192], "silver".to_string()),
      ([0, 255, 191], "aquamarine".to_string()),
      ([128, 0, 0], "maroon".to_string())
    ]);
    if colors.contains_key(&rgb) {
      return colors.get(&rgb).unwrap().to_string();
    } else {
      //by default lets say its white
      let mut closest: [u8; 3] = [255, 255, 255];
      for c in colors.keys() {
        let r_diff = (c[0] as i8-rgb[0] as i8).abs() as u16;
        let g_diff = (c[1] as i8-rgb[1] as i8).abs() as u16;
        let b_diff = (c[2] as i8-rgb[2] as i8).abs() as u16;
        let total_diff: u16 = r_diff+g_diff+b_diff;
        let r2_diff = (closest[0] as i8-rgb[0] as i8).abs() as u16;
        let g2_diff = (closest[1] as i8-rgb[1] as i8).abs() as u16;
        let b2_diff = (closest[2] as i8-rgb[2] as i8).abs() as u16;
        let total_diff2: u16 = r2_diff+g2_diff+b2_diff;
        if total_diff > total_diff2 {
          closest = *c;
        }
      }
      return "Similar to ".to_string()+colors.get(&closest).unwrap();
    }
  }
  pub fn composite_colors(color1: [u8; 4], color2: [u8; 4]) -> [u8; 4] {
    //convert the a from range 0-255 to range 0-1 (alpha_to_percentage)
    let a1 = BMP::alpha_to_percentage(color1[3]);
    let a2 = BMP::alpha_to_percentage(color2[3]);
    //alpha equation: a0 = a1 + a2(1-a1)
    let a0 = a1+a2*(1 as f64 - a1);
    //c0 = (c1a1 + c2a2(1 - a1)) / a0
    let c0_0: f64 = (color1[0] as f64*a1 + color2[0] as f64*a2*(1 as f64-a1))/a0;
    let c0_1: f64 = (color1[1] as f64*a1 + color2[1] as f64*a2*(1 as f64-a1))/a0;
    let c0_2: f64 = (color1[2] as f64*a1 + color2[2] as f64*a2*(1 as f64-a1))/a0;
    return [c0_0.round() as u8, c0_1.round() as u8, c0_2.round() as u8, BMP::percentage_to_alpha(a0)];
  }
  //file header related
  pub fn get_header(&self) -> BITMAPFILEHEADER {
    let header_bytes: &[u8; 14] = self.get_header_bytes();
    return BITMAPFILEHEADER {
      bfType: BMP::bytes_to_string(&header_bytes[..2]),
      bfSize: BMP::bytes_to_int(header_bytes[2..6].try_into().unwrap()),
      bfReserved1: header_bytes[6..8].try_into().unwrap(),
      bfReserved2: header_bytes[8..10].try_into().unwrap(),
      bfOffBits: BMP::bytes_to_int(header_bytes[10..14].try_into().unwrap()) as u32,
    };
  }
  fn get_header_bytes(&self) -> &[u8; 14] {
    //turn slice into array
    self.contents[..14].try_into().unwrap()
  }
  fn get_offset(&self) -> u32 {
    self.get_header().bfOffBits
  }
  pub fn get_size(&self, use_header: bool) -> u32 {
    if use_header {
      return self.get_header().bfSize;
    } else {
      return self.contents.len().try_into().unwrap();
    }
  }
  //dib header related
  pub fn get_dib_header(&self) -> Result<DIBHEADER, ErrorKind> {
    //this will not work because there may be other data besides the DIB header
    //let dib_size: i32 = self.get_offset()-14;
    //instead we will read the first 4 bytes after the header, which *should* specify the DIB header size, so we can figure out what kind of header it is
    let dib_size: u32 = BMP::bytes_to_int(self.contents[HEADER_OFFSET..HEADER_OFFSET+4].try_into().unwrap());
    let dib_header: DIBHEADER;
    match dib_size {
      12 => {
        //"BITMAPCOREHEADER"
        dib_header = DIBHEADER {
          size: dib_size,
          width: BMP::two_bytes_to_int(self.contents[HEADER_OFFSET+4..HEADER_OFFSET+6].try_into().unwrap()) as u32,
          height: BMP::two_bytes_to_signed_int(self.contents[HEADER_OFFSET+6..HEADER_OFFSET+8].try_into().unwrap()) as i32,
          planes: BMP::two_bytes_to_int(self.contents[HEADER_OFFSET+8..HEADER_OFFSET+10].try_into().unwrap()) as u16,
          bitcount: BMP::two_bytes_to_int(self.contents[HEADER_OFFSET+10..HEADER_OFFSET+12].try_into().unwrap()) as u16,
          compression: None,
          sizeimage: None,
          XPelsPerMeter: None,
          YPelsPerMeter: None,
          ClrUsed: None,
          ClrImportant: None,
          RedMask: None,
          GreenMask: None,
          BlueMask: None,
          AlphaMask: None,
          CSType: None,
          Endpoints: None,
          GammaRed: None,
          GammaGreen: None,
          GammaBlue: None,
          Intent: None,
          ProfileData: None,
          ProfileSize: None,
          Reserved: None,
        };
      },
      40 => {
        //"BITMAPINFOHEADER"
        dib_header = DIBHEADER {
          size: dib_size,
          width: BMP::bytes_to_int(self.contents[HEADER_OFFSET+4..HEADER_OFFSET+8].try_into().unwrap()),
          height: BMP::bytes_to_signed_int(self.contents[HEADER_OFFSET+8..HEADER_OFFSET+12].try_into().unwrap()),
          planes: BMP::two_bytes_to_int(self.contents[HEADER_OFFSET+12..HEADER_OFFSET+14].try_into().unwrap()) as u16,
          bitcount: BMP::two_bytes_to_int(self.contents[HEADER_OFFSET+14..HEADER_OFFSET+16].try_into().unwrap()) as u16,
          compression: Some(BMP::int_to_compression(BMP::bytes_to_int(self.contents[HEADER_OFFSET+16..HEADER_OFFSET+20].try_into().unwrap()))),
          sizeimage: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+20..HEADER_OFFSET+24].try_into().unwrap())),
          XPelsPerMeter: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+24..HEADER_OFFSET+28].try_into().unwrap())),
          YPelsPerMeter: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+28..HEADER_OFFSET+32].try_into().unwrap())),
          ClrUsed: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+32..HEADER_OFFSET+36].try_into().unwrap())),
          ClrImportant: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+36..HEADER_OFFSET+40].try_into().unwrap())),
          RedMask: None,
          GreenMask: None,
          BlueMask: None,
          AlphaMask: None,
          CSType: None,
          Endpoints: None,
          GammaRed: None,
          GammaGreen: None,
          GammaBlue: None,
          Intent: None,
          ProfileData: None,
          ProfileSize: None,
          Reserved: None,
        };
      },
      108 => {
        //"BITMAPV4HEADER"
        dib_header = DIBHEADER {
          size: dib_size,
          width: BMP::bytes_to_int(self.contents[HEADER_OFFSET+4..HEADER_OFFSET+8].try_into().unwrap()),
          height: BMP::bytes_to_signed_int(self.contents[HEADER_OFFSET+8..HEADER_OFFSET+12].try_into().unwrap()),
          planes: BMP::two_bytes_to_int(self.contents[HEADER_OFFSET+12..HEADER_OFFSET+14].try_into().unwrap()) as u16,
          bitcount: BMP::two_bytes_to_int(self.contents[HEADER_OFFSET+14..HEADER_OFFSET+16].try_into().unwrap()) as u16,
          compression: Some(BMP::int_to_compression(BMP::bytes_to_int(self.contents[HEADER_OFFSET+16..HEADER_OFFSET+20].try_into().unwrap()))),
          sizeimage: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+20..HEADER_OFFSET+24].try_into().unwrap())),
          XPelsPerMeter: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+24..HEADER_OFFSET+28].try_into().unwrap())),
          YPelsPerMeter: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+28..HEADER_OFFSET+32].try_into().unwrap())),
          ClrUsed: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+32..HEADER_OFFSET+36].try_into().unwrap())),
          ClrImportant: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+36..HEADER_OFFSET+40].try_into().unwrap())),
          RedMask: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+40..HEADER_OFFSET+44].try_into().unwrap())),
          GreenMask: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+44..HEADER_OFFSET+48].try_into().unwrap())),
          BlueMask: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+48..HEADER_OFFSET+52].try_into().unwrap())),
          AlphaMask: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+52..HEADER_OFFSET+56].try_into().unwrap())),
          CSType: Some(BMP::bytes_to_string(&self.contents[HEADER_OFFSET+56..HEADER_OFFSET+60])),
          //rgb
          Endpoints: Some([[BMP::bytes_to_signed_int(self.contents[HEADER_OFFSET+60..HEADER_OFFSET+64].try_into().unwrap()), BMP::bytes_to_signed_int(self.contents[HEADER_OFFSET+64..HEADER_OFFSET+68].try_into().unwrap()), BMP::bytes_to_signed_int(self.contents[HEADER_OFFSET+68..HEADER_OFFSET+72].try_into().unwrap())], [BMP::bytes_to_signed_int(self.contents[HEADER_OFFSET+72..HEADER_OFFSET+76].try_into().unwrap()), BMP::bytes_to_signed_int(self.contents[HEADER_OFFSET+76..HEADER_OFFSET+80].try_into().unwrap()), BMP::bytes_to_signed_int(self.contents[HEADER_OFFSET+80..HEADER_OFFSET+84].try_into().unwrap())], [BMP::bytes_to_signed_int(self.contents[HEADER_OFFSET+84..HEADER_OFFSET+88].try_into().unwrap()), BMP::bytes_to_signed_int(self.contents[HEADER_OFFSET+88..HEADER_OFFSET+92].try_into().unwrap()), BMP::bytes_to_signed_int(self.contents[HEADER_OFFSET+92..HEADER_OFFSET+96].try_into().unwrap())]]),
          GammaRed: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+96..HEADER_OFFSET+100].try_into().unwrap())),
          GammaGreen: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+100..HEADER_OFFSET+104].try_into().unwrap())),
          GammaBlue: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+104..HEADER_OFFSET+108].try_into().unwrap())),
          Intent: None,
          ProfileData: None,
          ProfileSize: None,
          Reserved: None,
        };
      },
      124 => {
        //"BITMAPV5HEADER"
        //dword 4 bytes
        //long 4 bytes
        //CIEXYZTRIPLE 36 bytes
        dib_header = DIBHEADER {
          size: dib_size,
          width: BMP::bytes_to_int(self.contents[HEADER_OFFSET+4..HEADER_OFFSET+8].try_into().unwrap()),
          height: BMP::bytes_to_signed_int(self.contents[HEADER_OFFSET+8..HEADER_OFFSET+12].try_into().unwrap()),
          planes: BMP::two_bytes_to_int(self.contents[HEADER_OFFSET+12..HEADER_OFFSET+14].try_into().unwrap()) as u16,
          bitcount: BMP::two_bytes_to_int(self.contents[HEADER_OFFSET+14..HEADER_OFFSET+16].try_into().unwrap()) as u16,
          compression: Some(BMP::int_to_compression(BMP::bytes_to_int(self.contents[HEADER_OFFSET+16..HEADER_OFFSET+20].try_into().unwrap()))),
          sizeimage: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+20..HEADER_OFFSET+24].try_into().unwrap())),
          XPelsPerMeter: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+24..HEADER_OFFSET+28].try_into().unwrap())),
          YPelsPerMeter: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+28..HEADER_OFFSET+32].try_into().unwrap())),
          ClrUsed: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+32..HEADER_OFFSET+36].try_into().unwrap())),
          ClrImportant: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+36..HEADER_OFFSET+40].try_into().unwrap())),
          RedMask: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+40..HEADER_OFFSET+44].try_into().unwrap())),
          GreenMask: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+44..HEADER_OFFSET+48].try_into().unwrap())),
          BlueMask: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+48..HEADER_OFFSET+52].try_into().unwrap())),
          AlphaMask: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+52..HEADER_OFFSET+56].try_into().unwrap())),
          CSType: Some(BMP::bytes_to_string(&self.contents[HEADER_OFFSET+56..HEADER_OFFSET+60])),
          //rgb
          Endpoints: Some([[BMP::bytes_to_signed_int(self.contents[HEADER_OFFSET+60..HEADER_OFFSET+64].try_into().unwrap()), BMP::bytes_to_signed_int(self.contents[HEADER_OFFSET+64..HEADER_OFFSET+68].try_into().unwrap()), BMP::bytes_to_signed_int(self.contents[HEADER_OFFSET+68..HEADER_OFFSET+72].try_into().unwrap())],  [BMP::bytes_to_signed_int(self.contents[HEADER_OFFSET+72..HEADER_OFFSET+76].try_into().unwrap()), BMP::bytes_to_signed_int(self.contents[HEADER_OFFSET+76..HEADER_OFFSET+80].try_into().unwrap()), BMP::bytes_to_signed_int(self.contents[HEADER_OFFSET+80..HEADER_OFFSET+84].try_into().unwrap())], [BMP::bytes_to_signed_int(self.contents[HEADER_OFFSET+84..HEADER_OFFSET+88].try_into().unwrap()), BMP::bytes_to_signed_int(self.contents[HEADER_OFFSET+88..HEADER_OFFSET+92].try_into().unwrap()), BMP::bytes_to_signed_int(self.contents[HEADER_OFFSET+92..HEADER_OFFSET+96].try_into().unwrap())]]),
          GammaRed: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+96..HEADER_OFFSET+100].try_into().unwrap())),
          GammaGreen: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+100..HEADER_OFFSET+104].try_into().unwrap())),
          GammaBlue: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+104..HEADER_OFFSET+108].try_into().unwrap())),
          Intent: Some(BMP::int_to_intent(BMP::bytes_to_int(self.contents[HEADER_OFFSET+108..HEADER_OFFSET+112].try_into().unwrap()))),
          //Some(BMP::bytes_to_string(&self.contents[HEADER_OFFSET+108..HEADER_OFFSET+112]))
          ProfileData: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+112..HEADER_OFFSET+116].try_into().unwrap()) as u16),
          ProfileSize: Some(BMP::bytes_to_int(self.contents[HEADER_OFFSET+116..HEADER_OFFSET+120].try_into().unwrap()) as u16),
          Reserved: Some(self.contents[HEADER_OFFSET+120..HEADER_OFFSET+124].try_into().unwrap()),
        };
      },
      _ => {
        //"unsupported"
        return Err(ErrorKind::Unsupported);
      },
    }
    return Ok(dib_header);
  }
  //extra bit masks
  fn get_extra_bit_masks(&self) -> Result<EXTRA_BIT_MASKS, ErrorKind> {
    //should be mutable instead of redefined, maybe
    let dib_header = self.get_dib_header();
    let dib_header = match dib_header {
      Ok(returned_dib_header) => returned_dib_header,
      Err(e) => return Err(e),
    };
    match dib_header.size {
      40 => {
        //see previous comment, should be mutable instead of redefined, maybe
        //offset should be 14+40
        const TOTAL_OFFSET: usize = 54;
        let compression = dib_header.compression.unwrap();
        if compression == "BI_BITFIELDS" {
          return Ok(EXTRA_BIT_MASKS::BI_BITFIELDS_MASKS(BI_BITFIELDS_MASKS {
            red: BMP::bytes_to_int(self.contents[TOTAL_OFFSET..TOTAL_OFFSET+4].try_into().unwrap()),
            green: BMP::bytes_to_int(self.contents[TOTAL_OFFSET+4..TOTAL_OFFSET+8].try_into().unwrap()),
            blue: BMP::bytes_to_int(self.contents[TOTAL_OFFSET+8..TOTAL_OFFSET+12].try_into().unwrap()),
          }));
        } else if compression == "BI_ALPHABITFIELDS" {
          return Ok(EXTRA_BIT_MASKS::BI_ALPHABITFIELDS_MASKS(BI_ALPHABITFIELDS_MASKS {
            red: BMP::bytes_to_int(self.contents[TOTAL_OFFSET..TOTAL_OFFSET+4].try_into().unwrap()),
            green: BMP::bytes_to_int(self.contents[TOTAL_OFFSET+4..TOTAL_OFFSET+8].try_into().unwrap()),
            blue: BMP::bytes_to_int(self.contents[TOTAL_OFFSET+8..TOTAL_OFFSET+12].try_into().unwrap()),
            alpha: BMP::bytes_to_int(self.contents[TOTAL_OFFSET+12..TOTAL_OFFSET+16].try_into().unwrap()),
          }));
        } else {
          return Err(ErrorKind::DoesNotExist);
        }
      },
      _ => return Err(ErrorKind::DoesNotExist),
    }
  }
  //color table
  //in between pixel array and everything else, I guess?
  //update: use the dib header's 'size' attribute - the actual size
  //return some kind of vector/array
  fn get_color_table(&self) -> Result<ColorTable, ErrorKind> {
    let dib_header = self.get_dib_header();
    let dib_header = match dib_header {
      Ok(returned_dib_header) => returned_dib_header,
      Err(e) => return Err(e),
    };
    //match (?) and extract header, get size
    //14 is the file header size
    let mut offset: u32 = 14;
    //where the actual pixel data starts, so the color table must end sometime before
    let end: u32;
    //either rgbtriple or masks or 
    let data_type: &str;
    //12, 40, 108, 124
    match dib_header.size {
      /*DIBHEADER::BITMAPCOREHEADER(b) | DIBHEADER::BITMAPINFOHEADER(b) | DIBHEADER::BITMAPV4HEADER(b) | DIBHEADER::BITMAPV5HEADER(b) => {
        size = b.size;
      }*/
      12 => {
        //https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapcoreinfo
        offset += dib_header.size;
        end = self.get_header().bfOffBits;
        //RGBTRIPLE, 3 bytes
        data_type = "rgbtriple";
      },
      40 | 108 | 124 => {
        //16 bit array instead of rgbquad is possible, but should not be used if file is "stored in a file or transferred to another application" https://www.digicamsoft.com/bmp/bmp.html
        offset += dib_header.size;
        end = self.get_header().bfOffBits;
        //https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapinfo
        //if compression is BI_RGB, using RGBQUAD 
        //size of array is biClrUsed
        let compression = dib_header.compression.unwrap();
        if compression == "BI_BITFIELDS" && (dib_header.bitcount == 16 || dib_header.bitcount == 32) {
          //extra bit masks, not color table. return error, or maybe extra bit masks? hmm
          return Err(ErrorKind::UseExtraBitMasks);
        } else if compression == "BI_RGB" && dib_header.bitcount >= 16 {
          //no color table
          //color table used for optimizing color palette or something instead, idk
          return Err(ErrorKind::DoesNotExist);
        } else {
          data_type = "rgbquad";
        }
      },
      _ => {
        return Err(ErrorKind::DoesNotExist);
      },
    };
    let color_table: ColorTable;
    if data_type == "rgbtriple" {
      let mut color_table_vec: Vec::<[u8; 3]> = Vec::new();
      //3 bytes
      for i in 0..(f64::from((end-offset)/3).floor() as i64) {
        let ii = i as u32;
        color_table_vec.push([BMP::byte_to_int(self.contents[(offset+ii*3) as usize]) as u8, BMP::byte_to_int(self.contents[(offset+ii*3+1) as usize]) as u8, BMP::byte_to_int(self.contents[(offset+ii*3+2) as usize]) as u8]);
      }
      color_table = ColorTable::RGBTRIPLE(color_table_vec);
    } else /*if "rgbquad" == data_type*/ {
      let mut color_table_vec: Vec::<[u8; 4]> = Vec::new();
      //4 bytes
      for i in 0..(f64::from((end-offset)/4).floor() as i64) {
        let ii = i as u32;
        color_table_vec.push([BMP::byte_to_int(self.contents[(offset+ii*4) as usize]) as u8, BMP::byte_to_int(self.contents[(offset+ii*4+1) as usize]) as u8, BMP::byte_to_int(self.contents[(offset+ii*4+2) as usize]) as u8, BMP::byte_to_int(self.contents[(offset+ii*4+3) as usize]) as u8]);
      }
      color_table = ColorTable::RGBQUAD(color_table_vec);
    }
    return Ok(color_table);
  }
  //pixel array
  pub fn get_pixel_data(&self) -> Result<VecDeque<Vec<Vec<u8>>>, ErrorKind> {
    //figure out if top down or bottom up
    //let it panic if it is an error
    let dib_header = self.get_dib_header();
    let dib_header = match dib_header {
      Ok(returned_dib_header) => returned_dib_header,
      Err(e) => return Err(e),
    };
    //figure out row size and image height
    //figure out pixel format
    //figure out is padded
    //monochrome is 1 bit per pixel. lets not support that for now
    //Vec<[[u8; dib_header.bitcount/4]; dib_header.width]>
    //change to array https://discord.com/channels/273534239310479360/273541522815713281/951356330696912967
    let mut rows: VecDeque<Vec<Vec<u8>>> = VecDeque::new();
    let header = self.get_header();
    if dib_header.height < 0 {
      //top down (starts from top left)
      //add rows as normal, to the back of vector
      //header.bfOffBits
      //https://en.wikipedia.org/wiki/BMP_file_format#Pixel_storage
      let row_length = f64::from(dib_header.bitcount as u16*dib_header.width as u16/32).ceil() as u32 * 4;
      //this may not work if there is profile data or other stuff after image?
      let rows_num = (self.contents.len() as u32-header.bfOffBits)/row_length;
      for row_num in 0..rows_num {
        //let row: Vec<[u8; dib_header.bitcount/4]> = Vec::new();
        let mut row: Vec<Vec<u8>> = Vec::new();
        for pixel in 0..dib_header.width {
          if dib_header.bitcount >= 8 {
            let start: u32 = (header.bfOffBits)+(row_num as u32)*row_length+(pixel as u32)*((dib_header.bitcount/8) as u32);
            row.push(self.contents[start as usize..(start+(dib_header.bitcount/8) as u32) as usize].to_vec());
          } else {
            //we need to do bitwise operators if the pixels are smaller than 1 byte size (1 bit, 2 bit, 4 bit)
            let start: u32 = (header.bfOffBits as u32)+(row_num as u32)*row_length+(pixel as u32)*(((dib_header.bitcount/8) as f64).ceil() as u32);
            let byte: u8 = self.contents[start as usize];
            if dib_header.bitcount == 1 {
              let split_bits: [u8; 8] = [byte >> 7, (byte & 0b01000000) >> 6, (byte & 0b00100000) >> 5, (byte & 0b00010000) >> 4, (byte & 0b00001000) >> 3, (byte & 0b00000100) >> 2, (byte & 0b00000010) >> 1, byte & 0b00000001];
              row.push(vec![split_bits[(pixel % ((8/dib_header.bitcount) as u32)) as usize]]);
            } else if dib_header.bitcount == 2 {
              let split_bits: [u8; 4] = [byte >> 6, (byte & 0b00110000) >> 4, (byte & 0b00001100) >> 2, byte & 0b00000011];
              row.push(vec![split_bits[(pixel % ((8/dib_header.bitcount) as u32)) as usize]]);
            } else if dib_header.bitcount == 4 {
              let split_bits: [u8; 2] = [byte >> 4, byte & 0b00001111];
              row.push(vec![split_bits[(pixel % ((8/dib_header.bitcount) as u32)) as usize]]);
            }
          }
        }
        rows.push_back(row);
      }
      //self.contents[]
    } else if dib_header.height > 0 {
      //bottom up (starts from lower left)
      //add rows to front of vector
      //let start: u32 = (header.bfOffBits as u32)+(row_num as u32)*row_length+(pixel as u32)*((dib_header.bitcount/8) as u32);
      let row_length = f64::from(dib_header.bitcount as u16*dib_header.width as u16/32).ceil() as u32 * 4;
      let rows_num = (self.contents.len() as u32-header.bfOffBits)/row_length;
      for row_num in 0..rows_num {
        let mut row: Vec<Vec<u8>> = Vec::new();
        for pixel in 0..dib_header.width {
          if dib_header.bitcount >= 8 {
            let start: u32 = (header.bfOffBits)+(row_num as u32)*row_length+(pixel as u32)*((dib_header.bitcount/8) as u32);
            row.push(self.contents[start as usize..(start+(dib_header.bitcount/8) as u32) as usize].to_vec());
          } else {
            //we need to do bitwise operators if the pixels are smaller than 1 byte size (1 bit, 2 bit, 4 bit)
            let start: u32 = (header.bfOffBits)+(row_num as u32)*row_length+(pixel as u32)*(((dib_header.bitcount/8) as f64).ceil() as u32);
            let byte: u8 = self.contents[start as usize];
            if dib_header.bitcount == 1 {
              let split_bits: [u8; 8] = [byte >> 7, (byte & 0b01000000) >> 6, (byte & 0b00100000) >> 5, (byte & 0b00010000) >> 4, (byte & 0b00001000) >> 3, (byte & 0b00000100) >> 2, (byte & 0b00000010) >> 1, byte & 0b00000001];
              row.push(vec![split_bits[(pixel % ((8/dib_header.bitcount) as u32)) as usize]]);
            } else if dib_header.bitcount == 2 {
              let split_bits: [u8; 4] = [byte >> 6, (byte & 0b00110000) >> 4, (byte & 0b00001100) >> 2, byte & 0b00000011];
              row.push(vec![split_bits[(pixel % ((8/dib_header.bitcount) as u32)) as usize]]);
            } else if dib_header.bitcount == 4 {
              let split_bits: [u8; 2] = [byte >> 4, byte & 0b00001111];
              row.push(vec![split_bits[(pixel % ((8/dib_header.bitcount) as u32)) as usize]]);
            }
          }
        }
        rows.push_front(row);
      }
    }
    return Ok(rows);
  }
  //location here is told
  //ICC color profile
  fn get_color_profile(&self) -> Result<Vec<u8>, ErrorKind> {
    //https://en.wikipedia.org/wiki/Color_management
    //CIEXYZTRIPLE ?
    //https://www.color.org/ICC_Minor_Revision_for_Web.pdf
    //seems pretty complex, and niche. We'll check if it exists, if so, return raw bytes data, otherwise return error
    let dib_header = self.get_dib_header();
    let dib_header = match dib_header {
      Ok(returned_dib_header) => returned_dib_header,
      Err(e) => return Err(e),
    };
    match dib_header.size {
      124 => {
        let cstype = dib_header.CSType.unwrap();
        if cstype == "PROFILE_EMBEDDED" || cstype == "PROFILE_LINKED" {
          return Ok(self.contents[dib_header.ProfileData.unwrap() as usize..].to_vec());
          //dib_header.ProfileData.unwrap()..dib_header.ProfileData.unwrap()+dib_header.ProfileSize.unwrap()
        } else {
          return Err(ErrorKind::DoesNotExist);
        }
      },
      _ => return Err(ErrorKind::DoesNotExist),
    }
    //Intent: String,
    //ProfileData: u16,
    //ProfileSize: u16,
    //CSType: String
  }
  //interpret color data
  //returns an array rgba (4 u8)
  pub fn get_color_of_px(&self, x: usize, y: usize) -> Result<[u8; 4], ErrorKind> {
    let dib_header = self.get_dib_header();
    let dib_header = match dib_header {
      Ok(returned_dib_header) => returned_dib_header,
      Err(e) => return Err(e),
    };
    //need to check if error
    let pixel_data = self.get_pixel_data();
    let pixel_data = match pixel_data {
      Ok(returned_pixel_data) => returned_pixel_data,
      Err(e) => return Err(e),
    };
    let pixel: &Vec<u8> = &pixel_data[y][x];
    let pixel: Vec<u8> = pixel.to_vec();
    //TODO: incorporate masks
    //if more than 12 bytes dib header, there are masks
    //RedMask, GreenMask, BlueMask, AlphaMask
    //if BI_BITFIELDS and 16 or 24 bits
    //also for smaller dib header (info), check to see if there are extra bit masks
    if dib_header.bitcount == 16 {
      let compression = dib_header.compression.unwrap();
      if compression == "BI_BITFIELDS" && (dib_header.RedMask.is_some() && dib_header.GreenMask.is_some() && dib_header.BlueMask.is_some()) {
        //check masks
        //due to complexity we dont actually use the masks, we convert them into integer, and then compare size. Bigger it is, the more the one is to the left
        let rgba: [u8; 4];
        //these should be from extra bit masks!
        let red_mask: u32 = dib_header.RedMask.unwrap();
        //let green_mask: u32 = dib_header.GreenMask.unwrap();
        let blue_mask: u32 = dib_header.BlueMask.unwrap();
        if red_mask < blue_mask {
          //assume rgb
          rgba = [BMP::byte_to_int(pixel[0]), BMP::byte_to_int(pixel[1]), BMP::byte_to_int(pixel[2]), 255];
        } else {
          //assume brg
          rgba = [BMP::byte_to_int(pixel[2]), BMP::byte_to_int(pixel[1]), BMP::byte_to_int(pixel[0]), 255];
        }
        return Ok(rgba);
      } else {
        //compression is "BI_RGB"
        //5 for each r,g,b (15 bits + 1 per pixel)
        //currently placeholder
        return Ok([0, 0, 0, 255]);
      }
    } else if dib_header.bitcount == 24 {
      //if 24 bit, no need to look at color table because it is rgb.
      //there is no alpha value, so it is 100 (nontransparent/opaque)
      //order is BGR not RGB
      let rgba: [u8; 4] = [BMP::byte_to_int(pixel[2]), BMP::byte_to_int(pixel[1]), BMP::byte_to_int(pixel[0]), 255];
      return Ok(rgba);
    } else if dib_header.bitcount == 32 {
      //32 means rgba
      let compression = dib_header.compression.unwrap();
      if (compression == "BI_BITFIELDS" || compression == "BI_ALPHABITFIELDS") && (dib_header.RedMask.is_some() && dib_header.GreenMask.is_some() && dib_header.BlueMask.is_some()) {
        //check masks
        //due to complexity we dont actually use the masks, we convert them into integer, and then compare size. Bigger it is, the more the one is to the left
        //placeholder
        //determine if alpha is in front or back. determine is rgb or brg
        let rgba: [u8; 4];
        let red_mask: u32 = dib_header.RedMask.unwrap();
        //let green_mask: u32 = dib_header.GreenMask.unwrap();
        let blue_mask: u32 = dib_header.BlueMask.unwrap();
        let alpha_mask: u32 = dib_header.AlphaMask.unwrap();
        if alpha_mask < red_mask {
          //alpha is in front
          if red_mask < blue_mask {
            //argb
            rgba = [BMP::byte_to_int(pixel[1]), BMP::byte_to_int(pixel[2]), BMP::byte_to_int(pixel[3]), BMP::byte_to_int(pixel[0])];
          } else {
            //abgr
            rgba = [BMP::byte_to_int(pixel[3]), BMP::byte_to_int(pixel[2]), BMP::byte_to_int(pixel[1]), BMP::byte_to_int(pixel[0])];
          }
        } else {
          //alpha is in back
          if red_mask < blue_mask {
            //rgba
            rgba = [BMP::byte_to_int(pixel[0]), BMP::byte_to_int(pixel[1]), BMP::byte_to_int(pixel[2]), BMP::byte_to_int(pixel[3])];
          } else {
            //bgra
            rgba = [BMP::byte_to_int(pixel[2]), BMP::byte_to_int(pixel[1]), BMP::byte_to_int(pixel[0]), BMP::byte_to_int(pixel[3])];
          }
        }
        return Ok(rgba);
      } else {
        let rgba: [u8; 4] = [BMP::byte_to_int(pixel[0]), BMP::byte_to_int(pixel[1]), BMP::byte_to_int(pixel[2]), BMP::byte_to_int(pixel[3])];
        return Ok(rgba);
      }
    } else {
      //otherwise look at color table for corresponding color. The bit (s) in the pixel data are indexes. We look up the index in the color table to find the color
      let color_table = self.get_color_table();
      let color_table = match color_table {
        Ok(returned_color_table) => returned_color_table,
        Err(e) => return Err(e),
      };
      //1, 2, 4 (half byte), 8 (1 bytes), 16 (2 bytes)
      let index;
      if dib_header.bitcount == 16 {
        index = BMP::two_bytes_to_int(BMP::vec_to_2u8_array(pixel));
      } else {
        index = BMP::byte_to_int(pixel[0]) as u16;
      }
      let rgba: [u8; 4];
      match color_table {
        ColorTable::RGBTRIPLE(vec) => {
          let rgb: [u8; 3] = vec[index as usize];
          //the array is fixed size [u8; 3] we want to turn it into [u8; 4] with the 4th being 255
          let mut rgb = rgb.to_vec();
          rgb.push(255);
          rgba = BMP::vec_to_4u8_array(rgb);
        },
        ColorTable::RGBQUAD(vec) => {
          rgba = vec[index as usize];
        }
      }
      return Ok(rgba);
    }
  }
  //more efficient version
  pub fn get_color_of_px_efficient(&self, x: usize, y: usize, dib_header: DIBHEADER, pixel_data: VecDeque<Vec<Vec<u8>>>, color_table: Option<ColorTable>) -> Result<[u8; 4], ErrorKind> {
    let pixel: &Vec<u8> = &pixel_data[y][x];
    let pixel: Vec<u8> = pixel.to_vec();
    //TODO: incorporate masks
    //if more than 12 bytes dib header, there are masks
    //RedMask, GreenMask, BlueMask, AlphaMask
    //if BI_BITFIELDS and 16 or 24 bits
    //also for smaller dib header (info), check to see if there are extra bit masks
    if dib_header.bitcount == 16 {
      let compression = dib_header.compression.unwrap();
      if compression == "BI_BITFIELDS" && (dib_header.RedMask.is_some() && dib_header.GreenMask.is_some() && dib_header.BlueMask.is_some()) {
        //check masks
        //due to complexity we dont actually use the masks, we convert them into integer, and then compare size. Bigger it is, the more the one is to the left
        let rgba: [u8; 4];
        //these should be from extra bit masks!
        let red_mask: u32 = dib_header.RedMask.unwrap();
        //let green_mask: u32 = dib_header.GreenMask.unwrap();
        let blue_mask: u32 = dib_header.BlueMask.unwrap();
        if red_mask < blue_mask {
          //assume rgb
          rgba = [BMP::byte_to_int(pixel[0]), BMP::byte_to_int(pixel[1]), BMP::byte_to_int(pixel[2]), 255];
        } else {
          //assume brg
          rgba = [BMP::byte_to_int(pixel[2]), BMP::byte_to_int(pixel[1]), BMP::byte_to_int(pixel[0]), 255];
        }
        return Ok(rgba);
      } else {
        //compression is "BI_RGB"
        //5 for each r,g,b (15 bits + 1 per pixel)
        //currently placeholder
        return Ok([0, 0, 0, 255]);
      }
    } else if dib_header.bitcount == 24 {
      //if 24 bit, no need to look at color table because it is rgb.
      //there is no alpha value, so it is 100 (nontransparent/opaque)
      //order is BGR not RGB
      let rgba: [u8; 4] = [BMP::byte_to_int(pixel[2]), BMP::byte_to_int(pixel[1]), BMP::byte_to_int(pixel[0]), 255];
      return Ok(rgba);
    } else if dib_header.bitcount == 32 {
      //32 means rgba
      let compression = dib_header.compression.unwrap();
      if (compression == "BI_BITFIELDS" || compression == "BI_ALPHABITFIELDS") && (dib_header.RedMask.is_some() && dib_header.GreenMask.is_some() && dib_header.BlueMask.is_some()) {
        //check masks
        //due to complexity we dont actually use the masks, we convert them into integer, and then compare size. Bigger it is, the more the one is to the left
        //placeholder
        //determine if alpha is in front or back. determine is rgb or brg
        let rgba: [u8; 4];
        let red_mask: u32 = dib_header.RedMask.unwrap();
        //let green_mask: u32 = dib_header.GreenMask.unwrap();
        let blue_mask: u32 = dib_header.BlueMask.unwrap();
        let alpha_mask: u32 = dib_header.AlphaMask.unwrap();
        if alpha_mask < red_mask {
          //alpha is in front
          if red_mask < blue_mask {
            //argb
            rgba = [BMP::byte_to_int(pixel[1]), BMP::byte_to_int(pixel[2]), BMP::byte_to_int(pixel[3]), BMP::byte_to_int(pixel[0])];
          } else {
            //abgr
            rgba = [BMP::byte_to_int(pixel[3]), BMP::byte_to_int(pixel[2]), BMP::byte_to_int(pixel[1]), BMP::byte_to_int(pixel[0])];
          }
        } else {
          //alpha is in back
          if red_mask < blue_mask {
            //rgba
            rgba = [BMP::byte_to_int(pixel[0]), BMP::byte_to_int(pixel[1]), BMP::byte_to_int(pixel[2]), BMP::byte_to_int(pixel[3])];
          } else {
            //bgra
            rgba = [BMP::byte_to_int(pixel[2]), BMP::byte_to_int(pixel[1]), BMP::byte_to_int(pixel[0]), BMP::byte_to_int(pixel[3])];
          }
        }
        return Ok(rgba);
      } else {
        let rgba: [u8; 4] = [BMP::byte_to_int(pixel[0]), BMP::byte_to_int(pixel[1]), BMP::byte_to_int(pixel[2]), BMP::byte_to_int(pixel[3])];
        return Ok(rgba);
      }
    } else {
      //otherwise look at color table for corresponding color. The bit (s) in the pixel data are indexes. We look up the index in the color table to find the color
      let color_table = match color_table {
        Some(returned_color_table) => returned_color_table,
        None => return Err(ErrorKind::Missing),
      };
      //1, 2, 4 (half byte), 8 (1 bytes), 16 (2 bytes)
      let index;
      if dib_header.bitcount == 16 {
        index = BMP::two_bytes_to_int(BMP::vec_to_2u8_array(pixel));
      } else {
        index = BMP::byte_to_int(pixel[0]) as u16;
      }
      let rgba: [u8; 4];
      match color_table {
        ColorTable::RGBTRIPLE(vec) => {
          let rgb: [u8; 3] = vec[index as usize];
          //the array is fixed size [u8; 3] we want to turn it into [u8; 4] with the 4th being 255
          let mut rgb = rgb.to_vec();
          rgb.push(255);
          rgba = BMP::vec_to_4u8_array(rgb);
        },
        ColorTable::RGBQUAD(vec) => {
          rgba = vec[index as usize];
        }
      }
      return Ok(rgba);
    }
  }
  //edit color pixels, only supports 24 and 32 bit
  pub fn change_color_of_pixel(&mut self, x: u16, mut y: u16, new_color: [u8; 4]) -> Result<(), ErrorKind> {
    //NEW_COLOR IS FLIPPED! See get color from pixel and get the correct order like in get_color_of_px
    //todo: top down or bottom down?
    let dib_header = self.get_dib_header();
    let dib_header = match dib_header {
      Ok(returned_dib_header) => returned_dib_header,
      Err(e) => return Err(e),
    };
    let header = self.get_header();
    //bits per pixel
    let bitcount = dib_header.bitcount;
    //only 24 and 32 bit
    if bitcount != 24 && bitcount != 32 {
      //return error
      return Err(ErrorKind::Unsupported);
    }
    //depending on if top down or bottom up, adjust  y
    if dib_header.height > 0 {
      //bottom up
      y = dib_header.height as u16 - y - 1;
    }
    //calculate row width (bytes)
    let row_length = (f64::from((bitcount/8) as u16*dib_header.width as u16/4).ceil() as u32 * 4) as u16;
    //amount of rows in front = y
    //add offset bits: header.bfOffBits (actually bytes)
    let start = y*row_length+header.bfOffBits as u16+(bitcount/8)*x;
    //get indexes to change
    //self.contents
    //change the contents
    if bitcount == 24 {
      //order is BGR not RGB
      //3 bytes
      self.contents[start as usize] = new_color[2];
      self.contents[(start+1) as usize] = new_color[1];
      self.contents[(start+2) as usize] = new_color[0];
    } else if bitcount == 32 {
      let red_mask: u32 = dib_header.RedMask.unwrap();
      //let green_mask: u32 = dib_header.RedMask.unwrap();
      let blue_mask: u32 = dib_header.BlueMask.unwrap();
      let alpha_mask: u32 = dib_header.AlphaMask.unwrap();
      //4 bytes
      if alpha_mask < red_mask {
        //alpha in front
        if red_mask < blue_mask {
          //argb
          self.contents[start as usize] = new_color[3];
          self.contents[(start+1) as usize] = new_color[0];
          self.contents[(start+2) as usize] = new_color[1];
          self.contents[(start+3) as usize] = new_color[2];
        } else {
          //abgr
          self.contents[start as usize] = new_color[3];
          self.contents[(start+1) as usize] = new_color[2];
          self.contents[(start+2) as usize] = new_color[1];
          self.contents[(start+3) as usize] = new_color[0];
        }
      } else {
        //alpha in back
        if red_mask < blue_mask {
          //rgba
          self.contents[start as usize] = new_color[0];
          self.contents[(start+1) as usize] = new_color[1];
          self.contents[(start+2) as usize] = new_color[2];
          self.contents[(start+3) as usize] = new_color[3];
        } else {
          //bgra
          self.contents[start as usize] = new_color[2];
          self.contents[(start+1) as usize] = new_color[1];
          self.contents[(start+2) as usize] = new_color[0];
          self.contents[(start+3) as usize] = new_color[3];
        }
      }
    }
    return Ok(());
  }
  pub fn change_color_of_pixels(&mut self, pixels: Vec<[u16; 2]>, new_color: [u8; 4]) -> Result<(), ErrorKind> {
    //same as change_color_of_pixel but more efficient
    //see comments in that function for explanations
    let dib_header = self.get_dib_header();
    let dib_header = match dib_header {
      Ok(returned_dib_header) => returned_dib_header,
      Err(e) => return Err(e),
    };
    let header = self.get_header();
    let bitcount = dib_header.bitcount;
    if bitcount != 24 && bitcount != 32 {
      return Err(ErrorKind::Unsupported);
    }
    let row_length = (f64::from((bitcount/8) as u16*dib_header.width as u16/4).ceil() as u32 * 4) as u16;
    for pixel in pixels {
      let x = pixel[0];
      let mut y = pixel[1];
      if dib_header.height > 0 {
        y = dib_header.height as u16 - y - 1;
      }
      let start = y*row_length+header.bfOffBits as u16+(bitcount/8)*x;
      if bitcount == 24 {
        self.contents[start as usize] = new_color[2];
        self.contents[(start+1) as usize] = new_color[1];
        self.contents[(start+2) as usize] = new_color[0];
      } else if bitcount == 32 {
        let red_mask: u32 = dib_header.RedMask.unwrap();
        let blue_mask: u32 = dib_header.BlueMask.unwrap();
        let alpha_mask: u32 = dib_header.AlphaMask.unwrap();
        if alpha_mask < red_mask {
          if red_mask < blue_mask {
            //argb
            self.contents[start as usize] = new_color[3];
            self.contents[(start+1) as usize] = new_color[0];
            self.contents[(start+2) as usize] = new_color[1];
            self.contents[(start+3) as usize] = new_color[2];
          } else {
            //abgr
            self.contents[start as usize] = new_color[3];
            self.contents[(start+1) as usize] = new_color[2];
            self.contents[(start+2) as usize] = new_color[1];
            self.contents[(start+3) as usize] = new_color[0];
          }
        } else {
          if red_mask < blue_mask {
            //rgba
            self.contents[start as usize] = new_color[0];
            self.contents[(start+1) as usize] = new_color[1];
            self.contents[(start+2) as usize] = new_color[2];
            self.contents[(start+3) as usize] = new_color[3];
          } else {
            //bgra
            self.contents[start as usize] = new_color[2];
            self.contents[(start+1) as usize] = new_color[1];
            self.contents[(start+2) as usize] = new_color[0];
            self.contents[(start+3) as usize] = new_color[3];
          }
        }
      }
    }
    return Ok(());
  }
  //image editing functions
  pub fn draw_image(&mut self, x: u16, y: u16, bmp2: BMP)  -> Result<(), ErrorKind> {
    //get height and width
    let dib_header = bmp2.get_dib_header().unwrap();
    let bmp2_height = (dib_header.height).abs();
    let bmp2_width = dib_header.width;
    for i in 0..bmp2_height {
      for j in 0..bmp2_width {
        let new_pixel = [x+j as u16, y+i as u16];
        let old_color = self.get_color_of_px(i as usize, j as usize).unwrap();
        let new_color = bmp2.get_color_of_px(i as usize, j as usize).unwrap();
        if old_color[3] == 255 && new_color[3] == 255 {
          self.change_color_of_pixel(new_pixel[0], new_pixel[1], new_color)?;
        } else {
          self.change_color_of_pixel(new_pixel[0], new_pixel[1], BMP::composite_colors(new_color, old_color))?;
        }
      }
    }
    return Ok(());
  }
  pub fn change_opacity(&mut self, opacity: u8) -> Result<(), ErrorKind> {
    let dib_header = self.get_dib_header();
    let dib_header = match dib_header {
      Ok(returned_dib_header) => returned_dib_header,
      Err(e) => return Err(e),
    };
    let height: u16 = dib_header.height.abs() as u16;
    let width: u16 = dib_header.width as u16;
    //change every pixel
    for y in 0..height {
      for x in 0..width {
        //get pixel color
        let old_color = self.get_color_of_px(x as usize, y as usize);
        let old_color: [u8; 4] = match old_color {
          Ok(returned_color) => returned_color,
          Err(e) => return Err(e),
        };
        let new_fill: [u8; 4] = [old_color[0], old_color[1], old_color[2], opacity];
        //change pixel color
        self.change_color_of_pixel(x, y, new_fill)?;
      }
    }
    return Ok(());
  }
  pub fn invert(&mut self, invert_alpha: Option<bool>) -> Result<(), ErrorKind> {
    //invert colors of image
    let dib_header = self.get_dib_header();
    let dib_header = match dib_header {
      Ok(returned_dib_header) => returned_dib_header,
      Err(e) => return Err(e),
    };
    let height: u16 = dib_header.height.abs() as u16;
    let width: u16 = dib_header.width as u16;
    //change every pixel
    for y in 0..height {
      for x in 0..width {
        //get pixel color
        let old_color = self.get_color_of_px(x as usize, y as usize);
        let old_color: [u8; 4] = match old_color {
          Ok(returned_color) => returned_color,
          Err(e) => return Err(e),
        };
        let mut new_fill: [u8; 4] = [255 - old_color[0], 255 - old_color[1], 255 - old_color[2], old_color[3]];
        if invert_alpha.is_some() {
          let invert_alpha_unwrapped = invert_alpha.unwrap();
          if invert_alpha_unwrapped {
            new_fill = [255 - old_color[0], 255 - old_color[1], 255 - old_color[2], 255 - old_color[3]];
          }
        }
        //change pixel color
        self.change_color_of_pixel(x, y, new_fill)?;
      }
    }
    return Ok(());
  }
  //shape, line making functions
  pub fn draw_line(&mut self, fill: [u8; 4], p1: [u16; 2], p2: [u16; 2]) -> Result<(), ErrorKind> {
    if p1[0] == p2[0] {
      //x matches x, straight vertical line
      for ay in 0..(p2[1] as i16 - p1[1] as i16).abs() as u16 {
        //if p1 is below p2
        if p1[1] < p2[1] {
          self.change_color_of_pixel(p1[0], p1[1]+ay, fill)?;
        } else {
          self.change_color_of_pixel(p2[0], p2[1]+ay, fill)?;
        }
      }
    } else if p1[1] == p2[1] {
      //y matches y, straight horizontal line
      for ax in 0..(p2[0] as i16 - p1[0] as i16).abs() as u16 {
        //if p1 is to the left of p2
        if p1[0] < p2[0] {
          self.change_color_of_pixel(p1[0]+ax, p1[1], fill)?;
        } else {
          self.change_color_of_pixel(p2[0]+ax, p2[1], fill)?;
        }
      }
    } else {
      let vertical_diff: u16 = ((p2[1] as i16 -p1[1] as i16).abs() + 1) as u16;
      let horizontal_diff: u16 = ((p2[0] as i16 - p1[0] as i16).abs() + 1) as u16;
      //get left most point
      let leftmost_p;
      let rightmost_p;
      if p1[0] < p2[0] {
        leftmost_p = p1;
        rightmost_p = p2;
      } else {
        leftmost_p = p2;
        rightmost_p = p1;
      }
      let highest_p;
      let lowest_p;
      if p1[1] < p2[1] {
        highest_p = p1;
        lowest_p = p2;
      } else {
        highest_p = p2;
        lowest_p = p1;
      }
      //if vertical equal or more than 2, there will be middle segments
      if vertical_diff >= 2 {
        let middle_segment_length: u16;
        let two_ends_combined_length;
        if horizontal_diff >= vertical_diff {
          // middle segments = floor horizontal/vertical
          middle_segment_length = f64::from((horizontal_diff)/(vertical_diff)).floor() as u16;
          // two ends = horizontal - (middle segments * (vertical-2))
          two_ends_combined_length = horizontal_diff - (middle_segment_length*(vertical_diff-2));
        } else {
          //else vertical_diff > horizontal_diff
          // middle segments = floor vertical/horizontal
          middle_segment_length = f64::from((vertical_diff)/(horizontal_diff)).floor() as u16;
          // two ends = vertical - (middle segments * (horizontal-2))
          two_ends_combined_length = vertical_diff - (middle_segment_length*(horizontal_diff-2));
        }
        // each end should be two ends / 2
        // if two ends = 1, make first end 1 and subtract 1 from last segment and give to last end
        if two_ends_combined_length == 1 {
          //let end_segment_length = two_ends_combined_length/2;
          //first segment
          //leftmost_p
          self.change_color_of_pixel(leftmost_p[0], leftmost_p[1], fill)?;
          //middle segments
          for j in 0..(vertical_diff-2) {
            for ji in 0..middle_segment_length {
              //if last pixel of last segment
              if j == vertical_diff-3 && ji == middle_segment_length-1 {
                continue;
              }
              if highest_p == leftmost_p {
                self.change_color_of_pixel(leftmost_p[0]+ji+j*middle_segment_length, rightmost_p[1]+j, fill)?;
              } else {
                self.change_color_of_pixel(leftmost_p[0]+ji+j*middle_segment_length, rightmost_p[1]-j, fill)?;
              }
            }
          }
          //last segment
          self.change_color_of_pixel(rightmost_p[0], rightmost_p[1], fill)?;
        } else if horizontal_diff >= vertical_diff {
          let end_segment_length = two_ends_combined_length/2;
          //first segment
          //leftmost_p
          for i in 0..end_segment_length {
            self.change_color_of_pixel(leftmost_p[0]+i, leftmost_p[1], fill)?;
          }
          //middle segments
          for j in 0..(vertical_diff-2) {
            for ji in 0..middle_segment_length {
              if highest_p == leftmost_p {
                self.change_color_of_pixel(leftmost_p[0]+ji+j*middle_segment_length+end_segment_length, leftmost_p[1]+j+1, fill)?;
              } else {
                self.change_color_of_pixel(leftmost_p[0]+ji+j*middle_segment_length, rightmost_p[1]-j, fill)?;
              }
            }
          }
          //last segment
          for k in 0..end_segment_length {
            self.change_color_of_pixel(rightmost_p[0]-k, rightmost_p[1], fill)?;
          }
        } else if horizontal_diff < vertical_diff {
          let end_segment_length = two_ends_combined_length/2;
          //first segment
          //leftmost_p
          for i in 0..end_segment_length {
            self.change_color_of_pixel(leftmost_p[0], leftmost_p[1]+i, fill)?;
          }
          //middle segments
          for j in 0..(horizontal_diff-2) {
            for ji in 0..middle_segment_length {
              if highest_p == leftmost_p {
                //self.change_color_of_pixel(leftmost_p[0]+ji+j*middle_segment_length+end_segment_length, leftmost_p[1]+j+1, fill);
                self.change_color_of_pixel(leftmost_p[0]+j+1, leftmost_p[1]+ji+j*middle_segment_length+end_segment_length, fill)?;
              } else {
                //self.change_color_of_pixel(leftmost_p[0]+ji+j*middle_segment_length, rightmost_p[1]-j, fill);
                self.change_color_of_pixel(leftmost_p[0]-j, rightmost_p[1]+ji+j*middle_segment_length, fill)?;
              }
            }
          }
          //last segment
          for k in 0..end_segment_length {
            self.change_color_of_pixel(lowest_p[0], lowest_p[1]-k, fill)?;
          }
        }
      } else {
        //if vertical diff is 1, divide in half. if decimal, floor and ceil, those are the two segments. else, two segments are equal length
        let first_segment: u16 = (f64::from(horizontal_diff/2)).floor() as u16;
        let second_segment: u16 = (f64::from(horizontal_diff/2)).ceil() as u16;
        for i in 0..first_segment {
          self.change_color_of_pixel(leftmost_p[0]+i, leftmost_p[1], fill)?;
        }
        for j in 0..second_segment {
          self.change_color_of_pixel(rightmost_p[0]-j, rightmost_p[1], fill)?;
        }
      }
    }
    return Ok(());
  }
  //p1 is top left, p2 is top right
  pub fn draw_rectangle(&mut self, fill: Option<[u8; 4]>, stroke: Option<[u8; 4]>, p1: [u16; 2], p2: [u16; 2]) -> Result<(), ErrorKind> {
    if stroke.is_some() {
      let unwrapped_stroke = stroke.unwrap();
      //top left to top right
      self.draw_line(unwrapped_stroke, p1, [p2[0], p1[1]])?;
      //bottom left to bottom right
      self.draw_line(unwrapped_stroke, [p1[0], p2[1]], [p2[0]+1, p2[1]])?; //(hotizontal_diff-1)
      //top left to bottom left
      self.draw_line(unwrapped_stroke, p1, [p1[0], p2[1]])?;
      //top right to bottom right
      self.draw_line(unwrapped_stroke, [p2[0], p1[1]], [p2[0], p2[1]])?;
    }
    if fill.is_some() {
      let unwrapped_fill = fill.unwrap();
      let p1_mod = [p1[0]+1, p1[1]+1];
      let p2_mod = [p2[0]-1, p2[1]-1];
      //fill outline
      self.draw_line(unwrapped_fill, p1_mod, [p2_mod[0], p1_mod[1]])?;
      self.draw_line(unwrapped_fill, [p1_mod[0], p2_mod[1]], [p2_mod[0]+1, p2_mod[1]])?;
      self.draw_line(unwrapped_fill, p1_mod, [p1_mod[0], p2_mod[1]])?;
      self.draw_line(unwrapped_fill, [p2_mod[0], p1_mod[1]], [p2_mod[0], p2_mod[1]])?;
      //fill fill_bucket
      self.fill_bucket(unwrapped_fill, (p1[0]+2).into(), (p1[1]+2).into())?;
    }
    return Ok(());
  }
  pub fn draw_ellipse(&mut self, center: [u16; 2], xlength: u16, ylength: u16, stroke: [u8; 4], fill: Option<[u8; 4]>, guess: bool) -> Result<(), ErrorKind> {
    //x^2/a^2 + y^2/b^2 = 1
    //y = sqroot((1 - x^2 / a^2) * b^2)
    //plug in values from x-xlength to x+xlength and find y value, fill inside
    //aka a^2
    let xlength_2: f64 = i32::pow(xlength.into(), 2) as f64;
    //aka b^2
    let ylength_2: f64 = i32::pow(ylength.into(), 2) as f64;
    let mut prev_y = 0;
    for il in 1..xlength+1 {
      let c_x = il;
      let c_x_2: f64 = i32::pow(c_x.into(), 2) as f64;
      let y = (((1 as f64-c_x_2/xlength_2)*ylength_2) as f64).sqrt().round() as u16;
      self.change_color_of_pixel(center[0]-c_x, center[1]+y, stroke)?;
      self.change_color_of_pixel(center[0]-c_x, center[1]-y, stroke)?;
      let diff = (prev_y as i16-y as i16).abs() as u16;
      if diff > 1 && il != 1 && guess {
        for d in 1..diff+1 {
          self.change_color_of_pixel(center[0]-c_x+1, center[1]+y+d, stroke)?;
          self.change_color_of_pixel(center[0]-c_x+1, center[1]-y-d, stroke)?;
        }
      }
      prev_y = y;
    }
    for ir in 1..xlength+1 {
      let c_x = ir;
      let c_x_2: f64 = i32::pow(c_x.into(), 2) as f64;
      let y = (((1 as f64-c_x_2/xlength_2)*ylength_2) as f64).sqrt().round() as u16;
      self.change_color_of_pixel(center[0]+c_x, center[1]+y, stroke)?;
      self.change_color_of_pixel(center[0]+c_x, center[1]-y, stroke)?;
      let diff = (prev_y as i16-y as i16).abs() as u16;
      if diff > 1 && ir != 1 && guess {
        for d in 1..diff+1 {
          self.change_color_of_pixel(center[0]+c_x-1, center[1]+y+d, stroke)?;
          self.change_color_of_pixel(center[0]+c_x-1, center[1]-y-d, stroke)?;
        }
      }
      prev_y = y;
    }
    self.change_color_of_pixel(center[0], center[1]+ylength, stroke)?;
    self.change_color_of_pixel(center[0], center[1]-ylength, stroke)?;
    if fill.is_some() {
      let unwrapped_fill = fill.unwrap();
      self.fill_bucket(unwrapped_fill, center[0] as usize, center[1] as usize)?;
    }
    return Ok(());
  }
  pub fn fill_bucket(&mut self, fill: [u8; 4], x: usize, y: usize) -> Result<Vec<[u16; 2]>, ErrorKind> {
    //fill same color connected to the (x,y) with new paint
    //check up, down, left, right. If same color as initial square, add to queue. Iterate through queue, after iterating add to visit and repeat
    let dib_header = self.get_dib_header();
    let dib_header = match dib_header {
      Ok(returned_dib_header) => returned_dib_header,
      Err(e) => return Err(e),
    };
    let replace_color = self.get_color_of_px(x as usize, y as usize);
    let replace_color: [u8; 4] = match replace_color {
      Ok(returned_replace_color) => returned_replace_color,
      Err(e) => return Err(e),
    };
    let mut visited: Vec<[u16; 2]> = Vec::new();
    let mut queue: Vec<[u16; 2]> = Vec::new();
    queue.push([x as u16, y as u16]);
    while queue.len() > 0 {
      if visited.contains(&queue[0]) {
        queue.remove(0);
        continue;
      }
      let x2: u16 = queue[0][0];
      let y2: u16 = queue[0][1];
      //turn current coords into fill color
      //self.change_color_of_pixel(x2, y2, fill);
      //check is surrounding (up, down, left, right) are same color
      //check to make sure these coords exist. (get height, width)
      //remember, indexes start at 0
      if y2+1 < dib_header.height as u16 {
        let down_color = self.get_color_of_px(x2 as usize, (y2+1) as usize);
        let down_color: [u8; 4] = match down_color {
          Ok(returned_down_color) => returned_down_color,
          Err(e) => return Err(e),
        };
        if down_color == replace_color {
          queue.push([x2 as u16, y2+1 as u16]);
        }
      }
      if y2-1 > 0 {
        //does not go all the way to up color
        let up_color = self.get_color_of_px(x2 as usize, (y2-1) as usize);
        let up_color: [u8; 4] = match up_color {
          Ok(returned_up_color) => returned_up_color,
          Err(e) => return Err(e),
        };
        if up_color == replace_color {
          queue.push([x2 as u16, y2-1 as u16]);
        }
      }
      if x2-1 > 0 {
        let left_color = self.get_color_of_px((x2-1) as usize, y2 as usize);
        let left_color: [u8; 4] = match left_color {
          Ok(returned_left_color) => returned_left_color,
          Err(e) => return Err(e),
        };
        if left_color == replace_color {
          queue.push([x2-1 as u16, y2 as u16]);
        }
      }
      if x2+1 < dib_header.width as u16 {
        let right_color = self.get_color_of_px((x2+1) as usize, y2 as usize);
        let right_color: [u8; 4] = match right_color {
          Ok(returned_right_color) => returned_right_color,
          Err(e) => return Err(e),
        };
        if right_color == replace_color {
          queue.push([x2+1 as u16, y2 as u16]);
        }
      }
      //end
      visited.push(queue[0]);
      queue.remove(0);
    }
    //loop through visited
    for px in &visited {
      self.change_color_of_pixel(px[0], px[1], fill)?;
    }
    //&self.save_to_new("src/images/e2.bmp");
    return Ok(visited);
  }
  //save image functions
  pub fn save_to_new(self, file_path: &str) -> Result<(), ErrorKind> {
    let mut new_file = fs::File::create(&std::path::Path::new(file_path)).unwrap();
    let write_op = new_file.write_all(&self.contents);
    let write_op = match write_op {
      Ok(file) => return Ok(()),
      Err(e) => return Err(ErrorKind::FailedToWrite),
    };
  }
}

//https://docs.microsoft.com/en-us/windows/win32/wcs/basic-color-management-concepts

/*RGB to written color hash table*/