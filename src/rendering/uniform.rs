use super::ShaderProgram;
use gl::types::*;

pub trait Uniform {
    fn set_uniform(&self, program: &ShaderProgram, location: GLint);
}

pub struct UniformCacher<T>
where
    T: Uniform,
{
    pub(super) uniform: T,
    location: GLint,
}

impl<T> UniformCacher<T>
where
    T: Uniform,
{
    pub fn new(uniform: T, attribute: &str, shader: &ShaderProgram) -> Self {
        Self {
            uniform,
            location: shader.location(attribute),
        }
    }

    pub fn set(&self, shader: &ShaderProgram) {
        self.uniform.set_uniform(shader, self.location);
    }
}
