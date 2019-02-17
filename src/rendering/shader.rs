use super::ShaderKind;
use gl::types::*;
use std::ffi::CStr;
use std::ptr::null;

pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn frag(source: &CStr) -> Result<Self, ()> {
        Self::new(source, ShaderKind::Fragment)
    }

    pub fn vert(source: &CStr) -> Result<Self, ()> {
        Self::new(source, ShaderKind::Vertex)
    }

    pub fn new(source: &CStr, kind: ShaderKind) -> Result<Self, ()> {
        let id = unsafe { gl::CreateShader(kind as GLenum) };
        let source_ptr = source.as_ptr() as *const _;
        let mut success: GLint = 0;
        unsafe {
            gl::ShaderSource(id, 1, &source_ptr, null());
            gl::CompileShader(id);
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }
        match success {
            1 => Ok(Shader { id }),
            _ => Err(()),
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}
