use super::enumerations::{BaseInternalFormat, PixelFormat, PixelData, TextureKind};
use gl::types::*;

pub struct Texture {
    id: GLuint,
}

impl Texture {
    pub fn new<'a>(data: &'a [u8], width: usize, height: usize) -> TextureBuilder<'a> {
        TextureBuilder {
            data,
            width,
            height,
            mipmap: false,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

pub struct TextureBuilder<'a> {
    data: &'a [u8],
    width: usize,
    height: usize,
    mipmap: bool,
}

impl<'a> TextureBuilder<'a> {
    pub fn build(self) -> Texture {
        let id = unsafe {
            let mut id: GLuint = 0;
            gl::GenTextures(1, &mut id);
            id
        };
        let texture = Texture { id };
        texture.bind();
        let data_ptr = self.data.as_ptr() as *const _;
        unsafe {
            gl::TexImage2D(
                TextureKind::_2D as u32,
                0,
                BaseInternalFormat::RGB as i32,
                self.width as GLsizei,
                self.height as GLsizei,
                0,
                PixelFormat::RGB as u32,
                PixelData::UnsignedByte as u32,
                data_ptr,
            );
        }
        if self.mipmap {
            unsafe {
                gl::GenerateMipmap(TextureKind::_2D as u32);
            }
        }
        texture
    }

    pub fn mipmap(mut self) -> Self {
        self.mipmap = true;
        self
    }
}
