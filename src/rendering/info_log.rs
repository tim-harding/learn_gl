use super::{Shader, ShaderProgram};
use gl::types::*;
use std::ffi::CString;
use std::ptr;

enum LogKind {
    Shader,
    ShaderProgram,
}

pub struct InfoLog {
    id: u32,
    kind: LogKind,
}

impl InfoLog {
    pub fn shader(shader: &Shader) -> InfoLog {
        InfoLog {
            id: shader.id,
            kind: LogKind::Shader,
        }
    }

    pub fn shader_program(program: &ShaderProgram) -> InfoLog {
        InfoLog {
            id: program.id,
            kind: LogKind::ShaderProgram,
        }
    }

    pub fn as_string(&self) -> String {
        let length = self.length();
        let buffer = self.create_buffer(length as usize);
        let error_ptr = buffer.as_ptr() as *mut _;
        unsafe {
            self.log_getter()(self.id, length, ptr::null_mut(), error_ptr);
        };
        buffer.to_string_lossy().into_owned()
    }

    fn length(&self) -> GLint {
        let mut log_length: GLint = 0;
        unsafe {
            self.log_length_getter()(self.id, gl::INFO_LOG_LENGTH, &mut log_length);
        }
        log_length
    }

    fn create_buffer(&self, length: usize) -> CString {
        let mut buffer: Vec<u8> = Vec::with_capacity(length + 1);
        buffer.extend([b' '].iter().cycle().take(length));
        let error = unsafe { CString::from_vec_unchecked(buffer) };
        error
    }

    fn log_getter(&self) -> unsafe fn(u32, i32, *mut GLsizei, *mut GLchar) {
        match self.kind {
            LogKind::Shader => gl::GetShaderInfoLog,
            LogKind::ShaderProgram => gl::GetProgramInfoLog,
        }
    }

    fn log_length_getter(&self) -> unsafe fn(u32, GLenum, *mut GLint) {
        match self.kind {
            LogKind::Shader => gl::GetShaderiv,
            LogKind::ShaderProgram => gl::GetProgramiv,
        }
    }
}
