
use super::{ShaderProgram, Uniform, VertexArray};
use gl::types::*;
use std::collections::{
    hash_map::Entry::{Occupied, Vacant},
    HashMap,
};
use std::ffi::CString;
use std::ptr;

pub struct Pairing<'a> {
    shader: &'a ShaderProgram,
    vertex_array: &'a VertexArray,
    uniforms: HashMap<GLint, Box<Uniform>>,
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
            uniforms: HashMap::new(),
            element_count,
        }
    }

    pub fn uniform(&mut self, location: GLint, value: Box<Uniform>) {
        let entry = self.uniforms.entry(location);
        match entry {
            Occupied(entry) => {
                entry.replace_entry(value);
            }
            Vacant(entry) => {
                entry.insert(value);
            }
        };
    }

    pub fn draw(&self) {
        self.shader.bind();
        self.vertex_array.bind();
        for (location, value) in &self.uniforms {
            value.set(self.shader.id, *location);
        }
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
