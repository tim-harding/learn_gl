use super::enumerations::{BaseInternalFormat, PixelFormat, PixelData, TextureKind, WrapMode, TextureParameter, FilterMode};
use gl::types::*;

pub struct Texture {
    id: GLuint,
}

impl Texture {
    pub fn new<'a>(data: &'a [u8], width: usize, height: usize) -> TextureBuilder<'a> {
        TextureBuilder {
            data, width,
            height,
            mipmap: false,
            wrap: Wrap::default(),
            filter: Filter::default(),
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

struct Wrap {
    s: WrapMode,
    t: WrapMode,
    r: WrapMode,
}

impl Wrap {
    pub fn default() -> Self {
        Self{ s: WrapMode::Repeat, t: WrapMode::Repeat, r: WrapMode::Repeat }
    }
}

struct Filter {
    min: FilterMode,
    mag: FilterMode,
}

impl Filter {
    pub fn default() -> Self {
        Filter{ min: FilterMode::Linear, mag: FilterMode::Linear, }
    }
}

pub struct TextureBuilder<'a> {
    data: &'a [u8],
    width: usize,
    height: usize,
    mipmap: bool,
    wrap: Wrap,
    filter: Filter,
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
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, TextureParameter::WrapS as u32, self.wrap.s as i32);
            gl::TexParameteri(gl::TEXTURE_2D, TextureParameter::WrapR as u32, self.wrap.r as i32);
            gl::TexParameteri(gl::TEXTURE_2D, TextureParameter::WrapT as u32, self.wrap.t as i32);
            gl::TexParameteri(gl::TEXTURE_2D, TextureParameter::MagFilter as u32, self.filter.mag as i32);
            gl::TexParameteri(gl::TEXTURE_2D, TextureParameter::MinFilter as u32, self.filter.min as i32);
        }
        texture
    }

    pub fn mipmap(mut self) -> Self {
        self.mipmap = true;
        self
    }

    pub fn wrap_s(mut self, mode: WrapMode) -> Self {
        self.wrap.s = mode;
        self
    }

    pub fn wrap_r(mut self, mode: WrapMode) -> Self {
        self.wrap.r = mode;
        self
    }

    pub fn wrap_t(mut self, mode: WrapMode) -> Self {
        self.wrap.t = mode;
        self
    }

    pub fn filter_min(mut self, mode: FilterMode) -> Self {
        self.filter.min = mode;
        self
    }

    pub fn filter_mag(mut self, mode: FilterMode) -> Self {
        self.filter.mag = mode;
        self
    }
}
