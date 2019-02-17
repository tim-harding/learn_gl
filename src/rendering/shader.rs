use super::{InfoLog, ShaderKind};
use gl::types::*;
use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;

pub struct Shader {
    pub(super) id: u32,
}

impl Shader {
    pub fn frag(source: &str) -> Result<Self, String> {
        Self::new(source, ShaderKind::Fragment)
    }

    pub fn vert(source: &str) -> Result<Self, String> {
        Self::new(source, ShaderKind::Vertex)
    }

    pub fn new(source: &str, kind: ShaderKind) -> Result<Self, String> {
        let source_c = CString::new(source).unwrap();
        let id = unsafe { gl::CreateShader(kind as GLenum) };
        let source_ptr = source_c.as_ptr() as *const _;
        let success = unsafe {
            gl::ShaderSource(id, 1, &source_ptr, ptr::null());
            gl::CompileShader(id);
            let mut success: GLint = mem::uninitialized();
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
            success
        };
        let shader = Shader { id };
        match success {
            1 => Ok(shader),
            _ => Err(InfoLog::shader(&shader).as_string()),
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
