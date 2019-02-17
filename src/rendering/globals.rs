use super::ShaderProgram;
use std::ffi::CString;

pub fn clear(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe {
        gl::ClearColor(red, green, blue, alpha);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

pub fn set_uniform(value: f32, shader: &ShaderProgram, attribute: &str) {
    let attribute_c = CString::new(attribute).unwrap_or(CString::default());
    let attribute_ptr = attribute_c.as_ptr() as *const _;
    let location = unsafe { gl::GetUniformLocation(shader.id, attribute_ptr) };
    unsafe {
        gl::Uniform1f(location, value);
    }
}
