use super::ShaderProgram;
use gl::types::*;
use nalgebra_glm;

pub struct Uniform<'a>
{
    shader: &'a ShaderProgram,
    location: GLint,
}

impl<'a> Uniform<'a>
{
    pub fn new(attribute: &str, shader: &'a ShaderProgram) -> Self {
        Self {
            shader,
            location: shader.location(attribute),
        }
    }
}

pub trait UniformValue {
    fn set(&self, uniform: &Uniform);
}

impl UniformValue for nalgebra_glm::Vec1 {
    fn set(&self, uniform: &Uniform) {
        unsafe {
            gl::ProgramUniform1f(uniform.shader.id, uniform.location, self.x);
        }
    }
}

impl UniformValue for nalgebra_glm::Vec2 {
    fn set(&self, uniform: &Uniform) {
        unsafe {
            gl::ProgramUniform2f(uniform.shader.id, uniform.location, self.x, self.y);
        }
    }
}

impl UniformValue for nalgebra_glm::Vec3 {
    fn set(&self, uniform: &Uniform) {
        unsafe {
            gl::ProgramUniform3f(uniform.shader.id, uniform.location, self.x, self.y, self.z);
        }
    }
}

impl UniformValue for nalgebra_glm::Vec4 {
    fn set(&self, uniform: &Uniform) {
        unsafe {
            gl::ProgramUniform4f(uniform.shader.id, uniform.location, self.x, self.y, self.z, self.w);
        }
    }
}

impl UniformValue for nalgebra_glm::IVec1 {
    fn set(&self, uniform: &Uniform) {
        unsafe {
            gl::ProgramUniform1i(uniform.shader.id, uniform.location, self.x);
        }
    }
}

impl UniformValue for nalgebra_glm::IVec2 {
    fn set(&self, uniform: &Uniform) {
        unsafe {
            gl::ProgramUniform2i(uniform.shader.id, uniform.location, self.x, self.y);
        }
    }
}

impl UniformValue for nalgebra_glm::IVec3 {
    fn set(&self, uniform: &Uniform) {
        unsafe {
            gl::ProgramUniform3i(uniform.shader.id, uniform.location, self.x, self.y, self.z);
        }
    }
}

impl UniformValue for nalgebra_glm::IVec4 {
    fn set(&self, uniform: &Uniform) {
        unsafe {
            gl::ProgramUniform4i(uniform.shader.id, uniform.location, self.x, self.y, self.z, self.w);
        }
    }
}