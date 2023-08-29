# ascii_image

A simple Rust crate for transforming images into ascii art.

## Example

<img width="640px" height="360px" src="tests/data/images/landscape.jpg" />
<img width="640px" height="360px" src="repo_assets/landscape_ascii.png" />

```
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