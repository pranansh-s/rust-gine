use image::{ImageBuffer, Rgba, Rgb};

pub struct Texture2D {
    id: u32,
    pub width: u32,
    pub height: u32,
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}

pub enum TextureAlpha {
    RGB(ImageBuffer<Rgb<u8>, Vec<u8>>),
    RGBA(ImageBuffer<Rgba<u8>, Vec<u8>>),
}

impl Texture2D {
    pub unsafe fn new(width: u32, height: u32, tex_data: &Vec<u8>, internal_format: i32, image_format: u32, wrap_s: i32, wrap_t: i32, filter_min: i32, filter_max: i32) -> Result<Self, std::io::Error> {
        let mut id = 0;
        gl::GenTextures(1, &mut id);
        
        gl::BindTexture(gl::TEXTURE_2D, id);
        gl::TexImage2D(gl::TEXTURE_2D, 0, internal_format, width as i32, height as i32, 0, image_format, gl::UNSIGNED_BYTE, tex_data.as_ptr() as *const _);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap_s);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap_t);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, filter_min);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filter_max);

        // gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        gl::BindTexture(gl::TEXTURE_2D, 0);
        
        Ok(
            Self {
                id,
                width,
                height,
            }
        )
    }

    pub unsafe fn apply(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.id);
    }
}
