# BMP Rust
BMP Rust is a rust library to read and write .bmp Image files. It has zero dependencies.

## Install
Obviously, install Rust. Add bmp-rust to your Cargo.toml file

```toml
[dependencies]
bmp-rust ="0.2.4"
```

You can now use the crate:

```rust
use bmp_rust::bmp::BMP;
```

## Documentation
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
```

Look at the [source code](src/bmp.rs) or [tests/example](example/main.rs) for more functions, and their usage.

[docs.rs](https://docs.rs/bmp-rust/0.2.4/bmp_rust/bmp/index.html)
