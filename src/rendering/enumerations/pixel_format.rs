#[allow(dead_code)]
#[repr(u32)]
pub enum PixelFormat {
    R = gl::RED,
    RG = gl::RG,
    RGB = gl::RGB,
    BGR = gl::BGR,
    RGBA = gl::RGBA,
    BGRA = gl::BGRA,
    IntR = gl::RED_INTEGER,
    IntRG = gl::RG_INTEGER,
    IntRGB = gl::RGB_INTEGER,
    IntBGR = gl::BGR_INTEGER,
    IntRGBA = gl::RGBA_INTEGER,
    IntBGRA = gl::BGRA_INTEGER,
    StencilIndex = gl::STENCIL_INDEX,
    DepthComponent = gl::DEPTH_COMPONENT,
    DepthStencil = gl::DEPTH_STENCIL,
}