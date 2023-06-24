use crate::AsciiImageBuilder;
use crate::AsciiImage;

impl<'a> AsciiImageBuilder<'a> {
    pub fn density_chars(&mut self, chars: Vec<char>) -> &mut Self {
        self.density_chars = Some(chars);
        self
    }

    pub fn include_alpha(&mut self, alpha: bool) -> &mut Self {
        self.include_alpha = Some(alpha);
        self
    }

    pub fn build(&mut self) -> AsciiImage<'a> {
        if let Some(chars) = &self.density_chars {
            if chars.len() == 0 {
                self.density_chars = None;
            }
            else if chars.len() > 256 {
                self.density_chars = Some(chars[0..255].to_vec());
            }
            
        }   

        AsciiImage{
            image: self.image,
            density_chars: self.density_chars.clone().unwrap_or(vec!['.',',',':','+','*','?','%','#','@']),
            include_alpha: self.include_alpha.unwrap_or(true)
        }
    }
}