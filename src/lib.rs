use image::{DynamicImage, GenericImageView};

pub fn generate_text(image: &DynamicImage, include_alpha: bool) -> String {
    let mut result = String::new();
    let density_chars: [char; 9] = ['.',',',':','+','*','?','%','#','@'];
    let gray_image = image.to_luma_alpha8();

    for (x, y, pixel) in gray_image.enumerate_pixels() {
        //alpha can't be zero
        let alpha: f32 = if include_alpha {
            u8::MAX as f32 / pixel.0[1] as f32
        } else {
            1.0
        };
        let index: usize = ((pixel.0[0] as f32 / alpha)
            * (8 as f32 //8 is for density_chars.len()-1
                / u8::MAX as f32))
            .round() as usize;
        result.push(density_chars[index]);

        if (y + 1) != image.dimensions().1 && (x + 1) % image.dimensions().0 == 0 {
            result.push('\n');
        }
    }
    return result;
}

pub fn generate_text_with_density_chars(image: &DynamicImage, include_alpha: bool, density_chars: &Vec<char>) -> Option<String> {
    if density_chars.len() == 0 || density_chars.len() > 256 {
        return None;
    }

    let mut result = String::new();
    let gray_image = image.to_luma_alpha8();

    for (x, y, pixel) in gray_image.enumerate_pixels() {
        let alpha: f32 = if include_alpha {
            u8::MAX as f32 / pixel.0[1] as f32
        } else {
            1.0
        };
        let index: usize = ((pixel.0[0] as f32 / alpha)
            * (density_chars.len().saturating_sub(1) as f32
                / u8::MAX as f32))
            .round() as usize;
        result.push(*density_chars.get(index)?);

        if (y + 1) != image.dimensions().1 && (x + 1) % image.dimensions().0 == 0 {
            result.push('\n');
        }
    }
    return Some(result);
}