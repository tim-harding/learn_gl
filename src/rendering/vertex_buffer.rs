use super::{BufferKind, BufferUsage};
use gl::types::*;
use std::mem::{size_of, uninitialized};

pub struct VertexBuffer {
    id: u32,
    kind: GLenum,
}

impl VertexBuffer {
    pub fn new<'a, T>(vertices: &'a [T]) -> VertexBufferBuilder<'a, T> {
        VertexBufferBuilder {
            vertices,
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

pub struct VertexBufferBuilder<'a, T: 'a> {
    vertices: &'a [T],
    usage: BufferUsage,
    kind: BufferKind,
}

impl<'a, T> VertexBufferBuilder<'a, T> {
    pub fn usage(mut self, usage: BufferUsage) -> Self {
        self.usage = usage;
        self
    }

    pub fn kind(mut self, kind: BufferKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn build(self) -> VertexBuffer {
        let gl_kind = self.kind as GLenum;
        let gl_usage = self.usage as GLenum;
        let vertex_bytes = (self.vertices.len() * size_of::<T>()) as isize;
        let vertex_ptr = self.vertices.as_ptr() as *const GLvoid;
        let id = unsafe {
            let mut id: GLuint = uninitialized();
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl_kind, id);
            gl::BufferData(gl_kind, vertex_bytes, vertex_ptr, gl_usage);
            id
        };
        VertexBuffer { id, kind: gl_kind }
    }
}
