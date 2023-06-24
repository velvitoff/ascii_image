use crate::AsciiImage;
use crate::AsciiImageBuilder;
use image::GenericImageView;
use image::{DynamicImage};

impl<'a> AsciiImage<'a> {
    pub fn builder(image: &DynamicImage) -> AsciiImageBuilder {
        AsciiImageBuilder {
            image,
            density_chars: None,
            include_alpha: None
        }
    }

    pub fn generate_text_grayscale(&self) -> Option<String> {
        let mut result = String::new();
       // println!("density_Char len: {}", self.density_chars.len());
        if self.include_alpha {
            let gray_image = self.image.to_luma_alpha8();
            for (x, y, pixel) in gray_image.enumerate_pixels() {
                let index: usize = ((pixel.0[0] as f32 / (u8::MAX as f32 / pixel.0[1] as f32)) * (self.density_chars.len().clamp(0,255).saturating_sub(1) as f32 / u8::MAX as f32)).round() as usize;
                //println!("index: {}", index);
                result.push(*self.density_chars.get(index)?);
                if (y+1) != self.image.dimensions().1 && (x+1) % self.image.dimensions().0 == 0 {
                    result.push('\n');
                }
            }   
        }
        else {
            let gray_image = self.image.to_luma8();
            for (x, y, pixel) in gray_image.enumerate_pixels() {
                let index: usize = (pixel.0[0] as f32 * (self.density_chars.len().clamp(0,255).saturating_sub(1) as f32 / u8::MAX as f32)).round() as usize;
                result.push(*self.density_chars.get(index)?);
                if (y+1) != self.image.dimensions().1 && (x+1) % self.image.dimensions().0 == 0 {
                    result.push('\n');
                }
            }
        }

        Some(result)
    }

    pub fn generate_image_grayscale(&self) {

    }
}
