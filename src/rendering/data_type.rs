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
    Int_2_10_10_10_Rev = gl::INT_2_10_10_10_REV,
    UnsignedInt_2_10_10_10_Rev = gl::UNSIGNED_INT_2_10_10_10_REV,
    UnsignedInt_10_11_11_Rev = gl::UNSIGNED_INT_10F_11F_11F_REV,
}
