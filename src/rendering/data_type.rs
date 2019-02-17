#[allow(dead_code)]
#[repr(u32)]
pub enum DataType {
    Byte = gl::BYTE,
    UnsignedByte = gl::UNSIGNED_BYTE,
    Short = gl::SHORT,
    UnsignedShort = gl::UNSIGNED_SHORT,
    Int = gl::INT,
    UnsignedInt = gl::UNSIGNED_INT,
    HalfFloat = gl::HALF_FLOAT,
    Float = gl::FLOAT,
    Double = gl::DOUBLE,
    Fixed = gl::FIXED,
    Int2_10_10_10Rev = gl::INT_2_10_10_10_REV,
    Unsignedint2_10_10_10Rev = gl::UNSIGNED_INT_2_10_10_10_REV,
    Unsignedint10_11_11Rev = gl::UNSIGNED_INT_10F_11F_11F_REV,
}
