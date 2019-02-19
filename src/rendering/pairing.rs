use super::material::Material;
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
    element_count: i32,
    material: Option<&'a Material>,
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
            material: None,
        }
    }

    pub fn material(mut self, material: &'a Material) -> Self {
        self.material = Some(material);
        self
    }

    pub fn draw(&self) {
        self.shader.bind();
        self.vertex_array.bind();
        if let Some(ref material) = self.material {
            material.bind();
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
