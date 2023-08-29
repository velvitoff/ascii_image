use image::{io::Reader as ImageReader, DynamicImage, Rgba, RgbaImage};
use image_ascii::ImageGenerator;
use rusttype::Font;

#[test]
fn gen_image_basic() {
    let file = ImageReader::open("tests/data/results_image/cat.png")
        .unwrap()
        .decode()
        .unwrap();
    let image = ImageReader::open("tests/data/images/cat.jpg")
        .unwrap()
        .decode()
        .unwrap();

    let bytes = std::fs::read("src/fonts/Ubuntu-Regular.ttf").unwrap();
    let font = Font::try_from_bytes(&bytes).unwrap();
    let res = ImageGenerator::new(&image, &font)
        .set_scale_x(4)
        .set_scale_y(4)
        .generate();

    assert_eq!(res.as_raw(), file.to_rgba8().as_raw());

    //res.save("test.png").unwrap();
}

#[test]
fn gen_image_with_background() {
    let file = ImageReader::open("tests/data/results_image/cat_background.png")
        .unwrap()
        .decode()
        .unwrap();
    let image = ImageReader::open("tests/data/images/cat.jpg")
        .unwrap()
        .decode()
        .unwrap();

    let bytes = std::fs::read("src/fonts/Ubuntu-Regular.ttf").unwrap();
    let font = Font::try_from_bytes(&bytes).unwrap();

    let background = DynamicImage::ImageRgba8(RgbaImage::from_fn(256, 256, |x, y| {
        Rgba([x as u8, y as u8, x as u8, 255u8])
    }));
    let res = ImageGenerator::new(&image, &font)
        .set_scale_x(5)
        .set_scale_y(5)
        .set_background(image_ascii::ImageGeneratorBackground::Image(&background))
        .generate();

    assert_eq!(res.as_raw(), file.to_rgba8().as_raw());
}

#[test]
fn gen_image_reverse_colors() {
    let file = ImageReader::open("tests/data/results_image/cat_reverse.png")
        .unwrap()
        .decode()
        .unwrap();
    let image = ImageReader::open("tests/data/images/cat.jpg")
        .unwrap()
        .decode()
        .unwrap();

    let bytes = std::fs::read("src/fonts/Ubuntu-Regular.ttf").unwrap();
    let font = Font::try_from_bytes(&bytes).unwrap();

    let res = ImageGenerator::new(&image, &font)
        .set_scale_x(5)
        .set_scale_y(5)
        .set_text_color(image_ascii::ImageGeneratorTextColor::Color(Rgba([
            0u8, 0u8, 0u8, 255u8,
        ])))
        .set_background(image_ascii::ImageGeneratorBackground::Color(Rgba([
            255u8, 255u8, 255u8, 255u8,
        ])))
        .generate();

    assert_eq!(res.as_raw(), file.into_rgba8().as_raw());
}

#[test]
fn gen_image_with_colors() {
    let file = ImageReader::open("tests/data/results_image/cat_color.png")
        .unwrap()
        .decode()
        .unwrap();
    let image = ImageReader::open("tests/data/images/cat.jpg")
        .unwrap()
        .decode()
        .unwrap();

    let bytes = std::fs::read("src/fonts/Ubuntu-Regular.ttf").unwrap();
    let font = Font::try_from_bytes(&bytes).unwrap();

    let res = ImageGenerator::new(&image, &font)
        .set_scale_x(5)
        .set_scale_y(5)
        .set_text_color(image_ascii::ImageGeneratorTextColor::CopyFromImage)
        .generate();

    assert_eq!(res.as_raw(), file.into_rgba8().as_raw());
}
