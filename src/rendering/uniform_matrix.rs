use gl::types::*;
use super::{ShaderProgram};
use nalgebra_glm::*;
use std::any::TypeId;

type Setter = unsafe fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat);

const MATRIX_IDS: [TypeId; 9] = [
    TypeId::of::<Mat2>(),
    TypeId::of::<Mat3>(),
    TypeId::of::<Mat4>(),
    TypeId::of::<Mat2x3>(),
    TypeId::of::<Mat3x2>(),
    TypeId::of::<Mat2x4>(),
    TypeId::of::<Mat4x2>(),
    TypeId::of::<Mat3x4>(),
    TypeId::of::<Mat4x3>(),
];
const SETTERS: [Setter; 9] = [
    gl::ProgramUniformMatrix2fv,
    gl::ProgramUniformMatrix3fv,
    gl::ProgramUniformMatrix4fv,
    gl::ProgramUniformMatrix2x3fv,
    gl::ProgramUniformMatrix3x2fv,
    gl::ProgramUniformMatrix2x4fv,
    gl::ProgramUniformMatrix4x2fv,
    gl::ProgramUniformMatrix3x4fv,
    gl::ProgramUniformMatrix4x3fv,
];

pub struct UniformMatrix<'a, T> where T: 'static {
    shader: &'a ShaderProgram,
    location: GLint,
    pub uniforms: Vec<T>,
    setter: Setter,
}

impl<'a, T> UniformMatrix<'a, T> where T: 'static {
    pub fn new(attribute: &str, shader: &'a ShaderProgram, uniforms: Vec<T>) -> Option<Self> {
        let id = TypeId::of::<T>();
        if let Some(position) = MATRIX_IDS.iter().position(|item| *item == id) {
            let location = shader.location(attribute);
            let setter = SETTERS[position];
            Some(Self{ shader, location, uniforms, setter})
        } else {
            None
        }
    }

    pub fn set(&self, transpose: bool) { 
        unsafe {
            let transpose = transpose as u8;
            let count = self.uniforms.len() as i32;
            let uniforms_ptr = self.uniforms.as_ptr() as *const GLfloat;
            let setter = self.setter;
            setter(self.shader.id, self.location, count, transpose, uniforms_ptr);
        }
    }
}