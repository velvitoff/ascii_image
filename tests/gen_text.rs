use image::io::Reader as ImageReader;
use image_ascii::AsciiImage;

//use std::io::Write as _;
//let mut file = std::fs::File::create("hek.txt").unwrap();
//write!(&mut file, "{}", res).unwrap();

#[test]
fn gen_text_grayscale_diamond_sword() {
    let file = std::fs::read_to_string("tests/data/results_text/diamond_sword_grayscale.txt").unwrap();
    let image = ImageReader::open("tests/data/images/diamond_sword.png")
        .unwrap()
        .decode()
        .unwrap();

    let asc = AsciiImage::builder(&image)
        .include_alpha(true)
        .build();
    let res = asc.generate_text().unwrap();
    assert_eq!(res, file);

    let asc = AsciiImage::builder(&image)
        .include_alpha(false)
        .build();
    let res = asc.generate_text().unwrap();
    assert_eq!(res, file);
}

#[test]
fn gen_text_grayscale_rust_icon() {
    let file = std::fs::read_to_string("tests/data/results_text/rust_icon_grayscale.txt").unwrap();
    let image = ImageReader::open("tests/data/images/rust_icon.png")
        .unwrap()
        .decode()
        .unwrap();

    let asc = AsciiImage::builder(&image)
        .include_alpha(true)
        .build();
    let res = asc.generate_text().unwrap();
    assert_eq!(res, file);

    let asc = AsciiImage::builder(&image)
        .include_alpha(false)
        .build();
    let res = asc.generate_text().unwrap();
    assert_eq!(res, file);
}

#[test]
fn gen_text_grayscale_cactus() {
    let file = std::fs::read_to_string("tests/data/results_text/cactus_grayscale.txt").unwrap();
    let image = ImageReader::open("tests/data/images/cactus.png")
        .unwrap()
        .decode()
        .unwrap();

    let asc = AsciiImage::builder(&image)
        .include_alpha(true)
        .build();
    let res = asc.generate_text().unwrap();
    assert_eq!(res, file);

    let asc = AsciiImage::builder(&image)
        .include_alpha(false)
        .build();
    let res = asc.generate_text().unwrap();
    assert_eq!(res, file);
}

#[test]
fn gen_text_grayscale_every_blackwhite() {
    let file1 = std::fs::read_to_string("tests/data/results_text/every_blackwhite_grayscale.txt").unwrap();
    let file2 = std::fs::read_to_string("tests/data/results_text/every_blackwhite_grayscale_double_density.txt").unwrap();
    let image = ImageReader::open("tests/data/images/every_blackwhite.png")
        .unwrap()
        .decode()
        .unwrap();

    let asc = AsciiImage::builder(&image)
        .build();
    let res = asc.generate_text().unwrap();
    assert_eq!(res, file1);

    let asc = AsciiImage::builder(&image)
        .density_chars(vec!['.',',',':','+','*','?','%','#','@','.',',',':','+','*','?','%','#','@'])
        .build();
    let res = asc.generate_text().unwrap();
    assert_eq!(res, file2);
}

#[test]
fn gen_text_grayscale_transparent() {
    let file =
        std::fs::read_to_string("tests/data/results_text/transparent_something_grayscale.txt").unwrap();
    let file_no_alpha =
        std::fs::read_to_string("tests/data/results_text/transparent_something_grayscale_no_alpha.txt")
            .unwrap();
    let image = ImageReader::open("tests/data/images/transparent_something.png")
        .unwrap()
        .decode()
        .unwrap();

    let asc = AsciiImage::builder(&image)
        .include_alpha(true)
        .build();
    let res = asc.generate_text().unwrap();
    assert_eq!(res, file);

    let asc = AsciiImage::builder(&image)
        .include_alpha(false)
        .build();
    let res = asc.generate_text().unwrap();
    assert_eq!(res, file_no_alpha);
}
