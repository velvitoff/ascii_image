use image::io::Reader as ImageReader;
use image_ascii::TextGenerator;

//use std::io::Write as _;
//let mut file = std::fs::File::create("hek.txt").unwrap();
//write!(&mut file, "{}", res).unwrap();

#[test]
fn gen_text_diamond_sword() {
    let file = std::fs::read_to_string("tests/data/results_text/diamond_sword.txt").unwrap();
    let image = ImageReader::open("tests/data/images/diamond_sword.png")
        .unwrap()
        .decode()
        .unwrap();

    let res = TextGenerator::new(&image).generate();
    assert_eq!(res, file);

    let res = TextGenerator::new(&image)
        .set_include_alpha(false)
        .generate();
    assert_eq!(res, file);
}

#[test]
fn gen_text_rust_icon() {
    let file = std::fs::read_to_string("tests/data/results_text/rust_icon.txt").unwrap();
    let image = ImageReader::open("tests/data/images/rust_icon.png")
        .unwrap()
        .decode()
        .unwrap();

    let res = TextGenerator::new(&image).generate();
    assert_eq!(res, file);

    let res = TextGenerator::new(&image)
        .set_include_alpha(false)
        .generate();
    assert_eq!(res, file);
}

#[test]
fn gen_text_cactus() {
    let file = std::fs::read_to_string("tests/data/results_text/cactus.txt").unwrap();
    let image = ImageReader::open("tests/data/images/cactus.png")
        .unwrap()
        .decode()
        .unwrap();

    let res = TextGenerator::new(&image).generate();
    assert_eq!(res, file);

    let res = TextGenerator::new(&image)
        .set_include_alpha(false)
        .generate();
    assert_eq!(res, file);
}

#[test]
fn gen_text_every_blackwhite() {
    let file1 = std::fs::read_to_string("tests/data/results_text/every_blackwhite.txt").unwrap();
    let file2 =
        std::fs::read_to_string("tests/data/results_text/every_blackwhite_double_density.txt")
            .unwrap();
    let image = ImageReader::open("tests/data/images/every_blackwhite.png")
        .unwrap()
        .decode()
        .unwrap();

    let res = TextGenerator::new(&image).generate();
    assert_eq!(res, file1);

    let res = TextGenerator::new(&image)
        .set_density_chars(&vec![
            '.', ',', ':', '+', '*', '?', '%', '#', '@', '.', ',', ':', '+', '*', '?', '%', '#',
            '@',
        ])
        .unwrap()
        .generate();
    assert_eq!(res, file2);
}

#[test]
fn gen_text_something_transparent() {
    let file =
        std::fs::read_to_string("tests/data/results_text/transparent_something.txt").unwrap();
    let file_no_alpha =
        std::fs::read_to_string("tests/data/results_text/transparent_something_no_alpha.txt")
            .unwrap();
    let image = ImageReader::open("tests/data/images/transparent_something.png")
        .unwrap()
        .decode()
        .unwrap();

    let res = TextGenerator::new(&image).generate();
    assert_eq!(res, file);

    let res = TextGenerator::new(&image)
        .set_include_alpha(false)
        .generate();
    assert_eq!(res, file_no_alpha);
}

#[test]
fn gen_text_density_chars_len0() {
    let image = ImageReader::open("tests/data/images/rust_icon.png")
        .unwrap()
        .decode()
        .unwrap();

    let value = TextGenerator::new(&image)
        .set_density_chars(&vec![])
        .is_none();
    assert_eq!(value, true);
}

#[test]
fn gen_text_density_chars_len256() {
    let file = std::fs::read_to_string("tests/data/results_text/every_blackwhite_256.txt").unwrap();
    let image = ImageReader::open("tests/data/images/every_blackwhite.png")
        .unwrap()
        .decode()
        .unwrap();

    let chars_256: Vec<char> = vec![
        '.', ',', ':', '+', '*', '?', '%', '#', '@', '.', ',', ':', '+', '*', '?', '%', '#', '@',
        '.', ',', ':', '+', '*', '?', '%', '#', '@', '.', ',', ':', '+', '*', '?', '%', '#', '@',
        '.', ',', ':', '+', '*', '?', '%', '#', '@', '.', ',', ':', '+', '*', '?', '%', '#', '@',
        '.', ',', ':', '+', '*', '?', '%', '#', '@', '.', ',', ':', '+', '*', '?', '%', '#', '@',
        '.', ',', ':', '+', '*', '?', '%', '#', '@', '.', ',', ':', '+', '*', '?', '%', '#', '@',
        '.', ',', ':', '+', '*', '?', '%', '#', '@', '.', ',', ':', '+', '*', '?', '%', '#', '@',
        '.', ',', ':', '+', '*', '?', '%', '#', '@', '.', ',', ':', '+', '*', '?', '%', '#', '@',
        '.', ',', ':', '+', '*', '?', '%', '#', '@', '.', ',', ':', '+', '*', '?', '%', '#', '@',
        '.', ',', ':', '+', '*', '?', '%', '#', '@', '.', ',', ':', '+', '*', '?', '%', '#', '@',
        '.', ',', ':', '+', '*', '?', '%', '#', '@', '.', ',', ':', '+', '*', '?', '%', '#', '@',
        '.', ',', ':', '+', '*', '?', '%', '#', '@', '.', ',', ':', '+', '*', '?', '%', '#', '@',
        '.', ',', ':', '+', '*', '?', '%', '#', '@', '.', ',', ':', '+', '*', '?', '%', '#', '@',
        '.', ',', ':', '+', '*', '?', '%', '#', '@', '.', ',', ':', '+', '*', '?', '%', '#', '@',
        '.', ',', ':', '+', '*', '?', '%', '#', '@', '.', ',', ':', '+', '*', '?', '%', '#', '@',
        '.', ',', ':', '+',
    ];
    assert_eq!(chars_256.len(), 256);

    let res = TextGenerator::new(&image)
        .set_density_chars(&chars_256)
        .unwrap()
        .generate();
    assert_eq!(res, file);
}

#[test]
fn ascii_image_density_chars_len_too_large() {
    let image = ImageReader::open("tests/data/images/every_blackwhite.png")
        .unwrap()
        .decode()
        .unwrap();

    let chars_257: Vec<char> = Vec::from_iter((0..257).map(|_| '1'));
    let value = TextGenerator::new(&image).set_density_chars(&chars_257).is_none();
    assert_eq!(value, true);
}
