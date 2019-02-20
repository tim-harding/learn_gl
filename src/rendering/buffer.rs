use super::enumerations::{BufferKind, BufferUsage};
use gl::types::*;
use std::mem::{size_of, uninitialized};

pub struct Buffer {
    pub id: u32,
    kind: GLenum,
}

impl Buffer {
    pub fn new<'a, T>(elements: &'a [T]) -> BufferBuilder<'a, T> {
        BufferBuilder {
            elements,
            usage: BufferUsage::StaticDraw,
            kind: BufferKind::Array,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.kind, self.id);
        }
    }
}

pub struct BufferBuilder<'a, T: 'a> {
    elements: &'a [T],
    usage: BufferUsage,
    kind: BufferKind,
}

impl<'a, T> BufferBuilder<'a, T> {
    pub fn usage(mut self, usage: BufferUsage) -> Self {
        self.usage = usage;
        self
    }

    pub fn kind(mut self, kind: BufferKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn build(self) -> Buffer {
        let gl_kind = self.kind as GLenum;
        let gl_usage = self.usage as GLenum;
        let vertex_bytes = (self.elements.len() * size_of::<T>()) as isize;
        let vertex_ptr = self.elements.as_ptr() as *const GLvoid;
        let id = unsafe {
            let mut id: GLuint = uninitialized();
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl_kind, id);
            gl::BufferData(gl_kind, vertex_bytes, vertex_ptr, gl_usage);
            id
        };
        Buffer { id, kind: gl_kind }
    }
}
