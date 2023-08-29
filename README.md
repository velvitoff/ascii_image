# ascii_image

A simple Rust crate for transforming images into ascii art.
[Crates io](https://crates.io/crates/image_ascii)
[Github](https://github.com/velvitoff/ascii_image)

## Example
```rust
use image::{io::Reader as ImageReader, RgbaImage, DynamicImage, Rgba};
use rusttype::Font;
use image_ascii::ImageGenerator;

let image = ImageReader::open("tests/data/images/landscape.jpg")
    .unwrap()
    .decode()
    .unwrap();

let bytes = std::fs::read("src/fonts/Ubuntu-Regular.ttf").unwrap();
let font = Font::try_from_bytes(&bytes).unwrap();

let res = ImageGenerator::new(&image, &font)
    .set_scale_x(4)
    .set_scale_y(4)
    .set_text_color(image_ascii::ImageGeneratorTextColor::CopyFromImage)
    .generate();

let res = image::imageops::resize(
    &res,
    image.width(),
    image.height(),
    image::imageops::FilterType::CatmullRom,
);

res.save("landscape.png").unwrap();
```

<img width="640px" height="360px" src="https://github.com/velvitoff/ascii_image/blob/main/tests/data/images/landscape.jpg" />
<img width="640px" height="360px" src="https://github.com/velvitoff/ascii_image/blob/main/repo_assets/landscape_ascii.png" />
