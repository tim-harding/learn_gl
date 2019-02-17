use super::{ShaderProgram, VertexArray};
use std::ptr;

pub struct Pairing<'a> {
    shader: &'a ShaderProgram,
    vertex_array: &'a VertexArray,
    element_count: i32,
}

impl<'a> Pairing<'a> {
    pub fn new(
        shader: &'a ShaderProgram,
        vertex_array: &'a VertexArray,
        element_count: i32,
    ) -> Self {
        Self {
            shader,
            vertex_array,
            element_count,
        }
    }

    // pub fn uniform(mut self, )

    pub fn draw(&self) {
        self.shader.bind();
        self.vertex_array.bind();
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.element_count,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
        }
    }
}
