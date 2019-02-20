#[allow(dead_code)]
#[repr(u32)]
pub enum TextureKind {
    _1D = gl::TEXTURE_1D,
    _1DArray = gl::TEXTURE_1D_ARRAY,
    _2D = gl::TEXTURE_2D,
    _2DArray = gl::TEXTURE_2D_ARRAY,
    _2DMultisample = gl::TEXTURE_2D_MULTISAMPLE,
    _2DMultisampleArray = gl::TEXTURE_2D_MULTISAMPLE_ARRAY,
    _3D = gl::TEXTURE_3D,
    CubeMap = gl::TEXTURE_CUBE_MAP,
    CubeMapArray = gl::TEXTURE_CUBE_MAP_ARRAY,
    Rectangle = gl::TEXTURE_RECTANGLE,
}
