use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use rusttype::{Font, Scale};

#[derive(Debug)]
pub struct TextGenerator<'a> {
    image: &'a DynamicImage,
    include_alpha: bool,
    density_chars: &'a [char],
}

impl<'a> TextGenerator<'a> {
    pub fn new(image: &'a DynamicImage) -> Self {
        TextGenerator {
            image,
            include_alpha: true,
            density_chars: &['.', ',', ':', '+', '*', '?', '%', '#', '@'],
        }
    }

    pub fn set_include_alpha(&'a mut self, value: bool) -> &'a mut Self {
        self.include_alpha = value;
        self
    }

    pub fn set_density_chars(&'a mut self, value: &'a [char]) -> Option<&'a mut Self> {
        if value.len() == 0 || value.len() > 256 {
            return None;
        }
        self.density_chars = value;
        Some(self)
    }

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

#[derive(Debug)]
pub enum ImageGeneratorBackground<'a> {
    Color(Rgba<u8>),
    Image(&'a DynamicImage),
}

#[derive(Debug)]
pub enum ImageGeneratorTextColor {
    Color(Rgba<u8>),
    CopyFromImage
}

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

    pub fn include_alpha(&'a mut self, value: bool) -> &'a Self {
        self.include_alpha = value;
        self
    }

    pub fn set_density_chars(&'a mut self, value: &'a [char]) -> Option<&'a mut Self> {
        if value.len() == 0 || value.len() > 256 {
            return None;
        }
        self.density_chars = value;
        Some(self)
    }

    pub fn set_background(&'a mut self, value: ImageGeneratorBackground<'a>) -> &'a mut Self {
        self.background = value;
        self
    }

    pub fn set_text_color(&'a mut self, value: ImageGeneratorTextColor) -> &'a mut Self {
        self.text_color = value;
        self
    }

    pub fn set_font(&'a mut self, font: &'a Font) -> &'a mut Self {
        self.font = font;
        self
    }

    pub fn set_scale_x(&'a mut self, value: u32) -> &'a mut Self {
        self.scale_x = value;
        self
    }

    pub fn set_scale_y(&'a mut self, value: u32) -> &'a mut Self {
        self.scale_y = value;
        self
    }

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

    pub fn generate(&'a self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
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
