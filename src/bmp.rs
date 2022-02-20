use std::fs;
use std::convert::TryInto;

/*Documentation - important links
https://docs.microsoft.com/en-us/windows/win32/gdi/bitmap-header-types
https://en.wikipedia.org/wiki/BMP_file_format#File_structure
http://fileformats.archiveteam.org/wiki/BMP
*/

struct BITMAPFILEHEADER {
  bfType: String,
  bfSize: u32,
  bfReserved1: Vec<u8>,
  bfReserved2: Vec<u8>,
  bfOffBits: u16,
}

//DIB Headers
struct BITMAPCOREHEADER {
  bcSize: u16,
  bcWidth: u32,
  bcHeight: u32,
  bcPlanes: u16,
  bcBitCount: u16,
}

//if biCompression is BI_ALPHABITFIELDS or BI_BITFIELDS 
struct BITMAPINFOHEADER {
  biSize: u16,
  biWidth: u32,
  //biHeight can be negative
  biHeight: i32,
  biPlanes: u16,
  biBitCount: u16,
  biCompression: String,
  biSizeImage: u32,
  biXPelsPerMeter: u32,
  biYPelsPerMeter: u32,
  biClrUsed: u32,
  biClrImportant: u32,
}

struct BITMAPV4HEADER {
  bV4Size: u16,
  bV4Width: u32,
  //bV4Height can be negative
  bV4Height: i32,
  bV4Planes: u16,
  bV4BitCount: u16,
  bV4V4Compression: String,
  bV4SizeImage: u32,
  bV4XPelsPerMeter: u32,
  bV4YPelsPerMeter: u32,
  bV4ClrUsed: u32,
  bV4ClrImportant: u32,
  bV4RedMask: u32,
  bV4GreenMask: u32,
  bV4BlueMask: u32,
  bV4AlphaMask: u32,
  bV4CSType: String,
  //rgb
  bV4Endpoints: [[i32; 3]; 3],
  bV4GammaRed: u32,
  bV4GammaGreen: u32,
  bV4GammaBlue: u32,
}

/*
struct BITMAPV5HEADER {
  DWORD        bV5Size;
  LONG         bV5Width;
  LONG         bV5Height;
  WORD         bV5Planes;
  WORD         bV5BitCount;
  DWORD        bV5Compression;
  DWORD        bV5SizeImage;
  LONG         bV5XPelsPerMeter;
  LONG         bV5YPelsPerMeter;
  DWORD        bV5ClrUsed;
  DWORD        bV5ClrImportant;
  DWORD        bV5RedMask;
  DWORD        bV5GreenMask;
  DWORD        bV5BlueMask;
  DWORD        bV5AlphaMask;
  DWORD        bV5CSType;
  CIEXYZTRIPLE bV5Endpoints;
  DWORD        bV5GammaRed;
  DWORD        bV5GammaGreen;
  DWORD        bV5GammaBlue;
  DWORD        bV5Intent;
  DWORD        bV5ProfileData;
  DWORD        bV5ProfileSize;
  DWORD        bV5Reserved;
}
*/

enum DIBHEADER {
  BITMAPCOREHEADER,
  BITMAPINFOHEADER,
  BITMAPV4HEADER,
}

pub struct BMP {
  contents: Vec<u8>,
  from_file: bool,
  //bitmap_file_header: BITMAPFILEHEADER,
  //dib_header: DIBHEADER,
}

impl BMP {
  /*pub fn new() -> BMP {
    return BMP { contents: Vec::new(), from_file: false };
  }*/
  pub fn new_from_file(file_path: &str) -> BMP {
    let contents = fs::read(file_path)
      .expect("Error encountered");
    return BMP { contents: contents, from_file: true, };
  }
  //utilities
  fn bytes_to_int(bytes: [u8; 4]) -> u32 {
    u32::from_ne_bytes(bytes)
  }
  fn bytes_to_string(bytes: &[u8]) -> String {
    String::from_utf8_lossy(&bytes).to_string()
  }
  fn num_bytes_to_kilobytes(bytes: u32) -> u32 {
    //1024 bytes per kilobyte
    bytes/1024
  }
  //file header related
  fn get_header(&self) -> BITMAPFILEHEADER {
    let header_bytes: &[u8; 14] = self.get_header_bytes();
    return BITMAPFILEHEADER {
      bfType: BMP::bytes_to_string(&header_bytes[..2]),
      bfSize: BMP::bytes_to_int(header_bytes[2..6].try_into().unwrap()),
      bfReserved1: header_bytes[6..8].try_into().unwrap(),
      bfReserved2: header_bytes[8..10].try_into().unwrap(),
      bfOffBits: BMP::bytes_to_int(header_bytes[10..14].try_into().unwrap()) as u16,
    };
  }
  fn get_header_bytes(&self) -> &[u8; 14] {
    //turn slice into array
    self.contents[..14].try_into().unwrap()
  }
  fn get_offset(&self) -> u16 {
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
  fn get_dib_header(&self) {
    //this will not work because there may be other data besides the DIB header
    //let dib_size: i32 = self.get_offset()-14;
    //instead we will read the first 4 bytes after the header, which *should* specify the DIB header size, so we can figure out what kind of header it is
    let dib_size: u32 = BMP::bytes_to_int(self.contents[14..18].try_into().unwrap());
    match dib_size {
      12 => "BITMAPCOREHEADER",
      40 => "BITMAPINFOHEADER",
      108 => "BITMAPV4HEADER",
      124 => "BITMAPV5HEADER",
      _ => "invalid",
    };
  }
}