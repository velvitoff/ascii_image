use image_ascii::AsciiImage;
use image::io::Reader as ImageReader;

//use std::io::Write as _;
//let mut file = std::fs::File::create("hek.txt").unwrap();
//write!(&mut file, "{}", res).unwrap();

#[test]
fn image_to_text() {
    let image = ImageReader::open("tests/images/diamond_sword.png").unwrap().decode().unwrap();
    let asc = AsciiImage::builder(&image).include_alpha(true).build();
    let res = asc.generate_text_grayscale().unwrap();
    assert_eq!(res, ".............:::\n............:@@,\n...........:@%@,\n..........:@%@,.\n.........:@%#,..\n........:@%#,...\n..::...:#%#,....\n..:+:.:#%#,.....\n...:*,#*#,......\n...:**+#,.......\n....:+:,........\n...:+,::,.......\n..:*,.,,:,......\n::+,....,,......\n:+,.............\n,,,.............");
}
