use super::{Material, VertexArray};
use std::ptr;

pub struct Pairing<'a> {
    vertex_array: &'a VertexArray,
    element_count: i32,
}

impl<'a> Pairing<'a> {
    pub fn new(vertex_array: &'a VertexArray, element_count: i32) -> Self {
        Self {
            vertex_array,
            element_count,
        }
    }

    pub fn draw(&self) {
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
