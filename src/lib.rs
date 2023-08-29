//! A library to transform images into ASCII art. Both into text and new images consisting of letters.
//! 
//! # Examples
//! Generating text with default parameters
//! ```
//! use image::{io::Reader as ImageReader};
//! use image_ascii::TextGenerator;
//! 
//! let image = ImageReader::open("tests/data/images/diamond_sword.png")
//!     .unwrap()
//!     .decode()
//!     .unwrap();
//! 
//! let result: String = TextGenerator::new(&image).generate();
//! ```
//! 
//! Generating text with custom parameters
//! ```
//! use image::{io::Reader as ImageReader};
//! use image_ascii::TextGenerator;
//! 
//! let image = ImageReader::open("tests/data/images/diamond_sword.png")
//!     .unwrap()
//!     .decode()
//!     .unwrap();
//! 
//! let result: String = TextGenerator::new(&image)
//!     .set_include_alpha(false)
//!     .set_density_chars(&['.', '/', '%', '#'])
//!     .unwrap()
//!     .generate();
//! ```
//! 
//! Generating an image with default parameters
//! ```
//! use image::{io::Reader as ImageReader, RgbaImage};
//! use rusttype::Font;
//! use image_ascii::ImageGenerator;
//! 
//! let image = ImageReader::open("tests/data/images/diamond_sword.png")
//!     .unwrap()
//!     .decode()
//!     .unwrap();
//! 
//! let bytes = std::fs::read("src/fonts/Ubuntu-Regular.ttf").unwrap();
//! let font = Font::try_from_bytes(&bytes).unwrap();
//! 
//! let result: RgbaImage = ImageGenerator::new(&image, &font).generate();
//! ```
//! 
//! Generating an image with custom parameters
//! ```
//! use image::{io::Reader as ImageReader, RgbaImage, DynamicImage, Rgba};
//! use rusttype::Font;
//! use image_ascii::ImageGenerator;
//! 
//! let image = ImageReader::open("tests/data/images/diamond_sword.png")
//!     .unwrap()
//!     .decode()
//!     .unwrap();
//! 
//! let bytes = std::fs::read("src/fonts/Ubuntu-Regular.ttf").unwrap();
//! let font = Font::try_from_bytes(&bytes).unwrap();
//! 
//! let background = DynamicImage::ImageRgba8(RgbaImage::from_fn(64, 64, |x, y| {
//!     Rgba([x as u8, y as u8, x as u8, 255u8])
//! }));
//! 
//! let result: RgbaImage = ImageGenerator::new(&image, &font)
//!     .set_scale_x(5)
//!     .set_scale_y(5)
//!     .set_text_color(image_ascii::ImageGeneratorTextColor::CopyFromImage)
//!     .set_background(image_ascii::ImageGeneratorBackground::Image(&background))
//!     .generate();
//!     
//! ```

extern crate image;
extern crate rusttype;
use crate::image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
use crate::rusttype::{Font, Scale};

/// Structure used to generate ASCII art strings.
#[derive(Debug)]
pub struct TextGenerator<'a> {
    image: &'a DynamicImage,
    include_alpha: bool,
    density_chars: &'a [char],
}

impl<'a> TextGenerator<'a> {
    /// TextGenerator constructor. Accepts an image to be turned into ASCII.
    pub fn new(image: &'a DynamicImage) -> Self {
        TextGenerator {
            image,
            include_alpha: true,
            density_chars: &['.', ',', ':', '+', '*', '?', '%', '#', '@'],
        }
    }

    /// Set value of include_alpha.
    /// Include_alpha defines whether the algorithm should take into account alpha values of pixels.
    pub fn set_include_alpha(&'a mut self, value: bool) -> &'a mut Self {
        self.include_alpha = value;
        self
    }

    /// Set value of density_chars.
    /// Density_chars is an array of characters used to replace pixels based on their brightness.
    /// Default of density_chars is is ['.', ',', ':', '+', '*', '?', '%', '#', '@'].
    /// Returns None only if length of a new array is outside of 1..255 bounds.
    pub fn set_density_chars(&'a mut self, value: &'a [char]) -> Option<&'a mut Self> {
        if value.len() == 0 || value.len() > 256 {
            return None;
        }
        self.density_chars = value;
        Some(self)
    }

    /// Generates a String representing the image in ASCII art.
    pub fn generate(&'a self) -> String {
        let mut result = String::new();
        let gray_image = self.image.to_luma_alpha8();

        for (x, y, pixel) in gray_image.enumerate_pixels() {
            let alpha: f32 = if self.include_alpha {
                u8::MAX as f32 / pixel.0[1] as f32
            } else {
                1.0
            };
            let index: usize = ((pixel.0[0] as f32 / alpha)
                * (self.density_chars.len().saturating_sub(1) as f32 / u8::MAX as f32))
                .round() as usize;
            result.push(self.density_chars[index]);

            if (y + 1) != self.image.dimensions().1 && (x + 1) % self.image.dimensions().0 == 0 {
                result.push('\n');
            }
        }
        result
    }
}

/// Enumeration of background generation methods for ImageGenerator.
#[derive(Debug)]
pub enum ImageGeneratorBackground<'a> {
    /// Use solid color for background.
    Color(Rgba<u8>),
    /// Use a custom image for background.
    Image(&'a DynamicImage),
}

/// Enumeration of possible text color for ImageGenerator.
#[derive(Debug)]
pub enum ImageGeneratorTextColor {
    /// Use the same color for every character.
    Color(Rgba<u8>),
    /// Paint every character in the same color as the source pixel.
    CopyFromImage
}

/// Structure used to generate ASCII art images
#[derive(Debug)]
pub struct ImageGenerator<'a> {
    image: &'a DynamicImage,
    //background_image: DynamicImage,
    include_alpha: bool,
    density_chars: &'a [char],
    background: ImageGeneratorBackground<'a>,
    text_color: ImageGeneratorTextColor,
    font: &'a Font<'a>,
    scale_x: u32,
    scale_y: u32,
}

impl<'a> ImageGenerator<'a> {
    /// ImageGenerator constructor. Accept an image to be turned into ASCII art and a font.
    pub fn new(image: &'a DynamicImage, font: &'a Font) -> Self {
        ImageGenerator {
            image,
            include_alpha: true,
            density_chars: &['.', ',', ':', '+', '*', '?', '%', '#', '@'],
            background: ImageGeneratorBackground::Color(Rgba([0u8, 0u8, 0u8, 255u8])),
            text_color: ImageGeneratorTextColor::Color(Rgba([255u8, 255u8, 255u8, 255u8])),
            font,
            scale_x: 24,
            scale_y: 24,
        }
    }

    /// Set value of include_alpha.
    /// Include_alpha defines whether the algorithm should take into account alpha values of pixels
    pub fn include_alpha(&'a mut self, value: bool) -> &'a Self {
        self.include_alpha = value;
        self
    }

    /// Set value of density_chars.
    /// Density_chars is an array of characters used to replace pixels based on their brightness.
    /// Default of density_chars is is ['.', ',', ':', '+', '*', '?', '%', '#', '@'].
    /// Returns None only if length of a new array is outside of 1..255 bounds.
    pub fn set_density_chars(&'a mut self, value: &'a [char]) -> Option<&'a mut Self> {
        if value.len() == 0 || value.len() > 256 {
            return None;
        }
        self.density_chars = value;
        Some(self)
    }

    /// Set background for image. Can be a single color or a custom image.
    /// If custom image doesn't match the size of a generated image, the generated image gets resized to match the background.
    pub fn set_background(&'a mut self, value: ImageGeneratorBackground<'a>) -> &'a mut Self {
        self.background = value;
        self
    }

    /// Set text color for letters.
    pub fn set_text_color(&'a mut self, value: ImageGeneratorTextColor) -> &'a mut Self {
        self.text_color = value;
        self
    }

    /// Set horizontal scale for letters.
    pub fn set_scale_x(&'a mut self, value: u32) -> &'a mut Self {
        self.scale_x = value;
        self
    }

    /// Set vertical scale for letters.
    pub fn set_scale_y(&'a mut self, value: u32) -> &'a mut Self {
        self.scale_y = value;
        self
    }

    // Writes letters on top a received image.
    fn write_to_image(&'a self, image: &mut RgbaImage, text: &str) {
        let mut text_counter: usize = 0;
        for y in 0..self.image.height() {
            for x in 0..self.image.width() {
                let mut c = &text[text_counter..text_counter + 1];
                text_counter += 1;
                if c == "\n" {
                    c = &text[text_counter..text_counter + 1];
                    text_counter += 1;
                }
                imageproc::drawing::draw_text_mut(
                    image,
                    match self.text_color {
                        ImageGeneratorTextColor::Color(x) => x,
                        ImageGeneratorTextColor::CopyFromImage => self.image.get_pixel(x, y),
                    },
                    (x * self.scale_x + (self.scale_x / 2)) as i32,
                    (y * self.scale_y) as i32,
                    Scale {
                        x: self.scale_x as f32,
                        y: self.scale_y as f32,
                    },
                    self.font,
                    c,
                );
            }
        }
    }

    /// Generates an image.
    pub fn generate(&'a self) -> RgbaImage {
        let mut text_image: RgbaImage = match self.background {
            ImageGeneratorBackground::Color(c) => RgbaImage::from_fn(
                self.image.width() * self.scale_x,
                self.image.height() * self.scale_y,
                |_, _| c,
            ),
            ImageGeneratorBackground::Image(_) => RgbaImage::from_fn(
                self.image.width() * self.scale_x,
                self.image.height() * self.scale_y,
                |_, _| Rgba([0u8, 0u8, 0u8, 0u8]),
            ),
        };

        let text = TextGenerator::new(&self.image)
            .set_include_alpha(self.include_alpha)
            .set_density_chars(self.density_chars)
            .unwrap()
            .generate();

        match self.background {
            ImageGeneratorBackground::Color(_) => {
                self.write_to_image(&mut text_image, &text);
                return text_image;
            },
            ImageGeneratorBackground::Image(background) => {
                self.write_to_image(&mut text_image, &text);
                let text_image: RgbaImage = image::imageops::resize(&text_image, background.width(), background.height(), image::imageops::FilterType::CatmullRom);
                let mut copy: RgbaImage = background.clone().to_rgba8();
                image::imageops::overlay(&mut copy, &text_image, 0, 0);
                return copy;
            },
        }
    }
}
