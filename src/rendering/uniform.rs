use gl::types::*;

pub trait Uniform {
    fn set(&self, program: GLuint, location: GLint);
}
