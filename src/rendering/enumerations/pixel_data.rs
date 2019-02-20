#[allow(dead_code)]
#[repr(u32)]
pub enum PixelData {
    UnsignedByte = gl::UNSIGNED_BYTE,
    Byte = gl::BYTE,
    UnsignedShort = gl::UNSIGNED_SHORT,
    Short = gl::SHORT,
    UnsignedInt = gl::UNSIGNED_INT,
    Int = gl::INT,
    HalfFloat = gl::HALF_FLOAT,
    Float = gl::FLOAT,
}
