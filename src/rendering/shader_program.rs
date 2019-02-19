use super::{InfoLog, Shader};
use std::mem::uninitialized;
use std::ffi::CString;
use gl::types::*;

pub struct ShaderProgram {
    pub(super) id: u32,
}

impl ShaderProgram {
    pub fn new<'a>() -> ShaderProgramBuilder<'a> {
        ShaderProgramBuilder { shaders: vec![] }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn location(&self, attribute: &str) -> GLint {
        let attribute_c = CString::new(attribute).unwrap_or(CString::default());
        let attribute_ptr = attribute_c.as_ptr() as *const _;
        unsafe { gl::GetUniformLocation(self.id, attribute_ptr) }
    }
}

pub struct ShaderProgramBuilder<'a> {
    shaders: Vec<&'a Shader>,
}

impl<'a> ShaderProgramBuilder<'a> {
    pub fn with(mut self, shader: &'a Shader) -> Self {
        self.shaders.push(shader);
        self
    }

    pub fn build(&self) -> Result<ShaderProgram, String> {
        let id = unsafe { gl::CreateProgram() };
        for shader in self.shaders.iter() {
            unsafe {
                gl::AttachShader(id, shader.id);
            }
        }
        let success = unsafe {
            let mut success = uninitialized();
            gl::LinkProgram(id);
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
            success
        };
        let program = ShaderProgram { id };
        match success {
            1 => Ok(program),
            _ => Err(InfoLog::shader_program(&program).as_string()),
        }
    }
}
