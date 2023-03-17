# BMP Rust
BMP Rust is a rust library to read and write .bmp Image files. It has zero dependencies.

## Install
Obviously, install Rust. Add bmp-rust to your Cargo.toml file

```toml
[dependencies]
bmp-rust ="0.3.4"
```

You can now use the crate:

```rust
use bmp_rust::bmp::BMP;
```

## Documentation
The [docs.rs](https://docs.rs/bmp-rust/latest/bmp_rust/bmp/index.html) page contains documentation for all functions and types in this library.

First, load a BMP file by file path, or create a new one:
```rust
let mut bmp_from_file = BMP::new_from_file("midnight.bmp");
let mut bmp_from_scratch = BMP::new(15, 15, None);
```

Information can now be read from the file:
```rust
let file_size = bmp_from_file.get_size(true);
let dib_header = bmp_from_file.get_dib_header().unwrap();
let width = dib_header.width;
let height = dib_header.height;
let pixel_color = bmp_from_file.get_color_of_px(10, 10).unwrap();
```

Or new pixel data can be written to it:
```rust
bmp_from_file.change_color_of_pixel(10, 10, [233, 71, 255, 255]).expect("Failed to change color of pixel");
bmp_from_file.fill_bucket([155, 42, 66, 255], 35, 40).expect("Failed to bucket fill");
bmp_from_file.draw_line([100, 65, 45, 255], [20, 20], [52, 52]).expect("Failed to draw line");
bmp_from_file.draw_rectangle(None, Some([255, 255, 255, 255]), [0,2], [15,11]).expect("Failed to draw rect");
bmp_from_file.draw_ellipse([23, 25], 10, 12, [255, 0, 0, 255], Some([125, 64, 64, 255]), true).expect("Failed to draw ellipse");
bmp_from_file.invert(None).expect("Failed to invert");
bmp_from_file.change_opacity(25).expect("Failed to change opacity");
bmp_from_file.draw_image(5, 5, bmp_from_scratch).expect("Failed to draw image");
bmp_from_file.translate(-3, 5);
```

Look at the [source code](src/bmp.rs) or [tests/example](example/main.rs) for more functions, and their usage.
