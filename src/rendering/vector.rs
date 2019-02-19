use super::{ShaderProgram, Uniform};
use gl::types::*;

pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

impl Uniform for Vector4 {
    fn set_uniform(&self, program: &ShaderProgram, location: GLint) {
        unsafe {
            gl::ProgramUniform4f(program.id, location, self.x, self.y, self.z, self.w);
        }
    }
}

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn one() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl Uniform for Vector3 {
    fn set_uniform(&self, program: &ShaderProgram, location: GLint) {
        unsafe {
            gl::ProgramUniform3f(program.id, location, self.x, self.y, self.z);
        }
    }
}

pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Uniform for Vector2 {
    fn set_uniform(&self, program: &ShaderProgram, location: GLint) {
        unsafe {
            gl::ProgramUniform2f(program.id, location, self.x, self.y);
        }
    }
}

pub struct Unary {
    pub value: f32,
}

impl Unary {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

impl Uniform for Unary {
    fn set_uniform(&self, program: &ShaderProgram, location: GLint) {
        unsafe {
            gl::ProgramUniform1f(program.id, location, self.value);
        }
    }
}
