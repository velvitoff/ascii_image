use image::DynamicImage;
mod ascii_img_impl;
mod ascii_img_builder;

pub struct AsciiImage<'a> {
    image: &'a DynamicImage,
    density_chars: Vec<char>,
    include_alpha: bool
}

pub struct AsciiImageBuilder<'a> {
    image: &'a DynamicImage,
    density_chars: Option<Vec<char>>,
    include_alpha: Option<bool>
}
