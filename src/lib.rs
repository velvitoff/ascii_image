use image::{DynamicImage, GenericImageView};

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

    pub fn set_include_alpha(&'a mut self, value: bool) -> &'a Self {
        self.include_alpha = value;
        self
    }

    pub fn set_density_chars(&'a mut self, value: &'a [char]) -> Option<&'a Self> {
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



