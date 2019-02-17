use super::{Buffer, DataType, ShaderProgram};
use gl::types::*;
use std::ffi::{self, CString};
use std::mem;

pub struct VertexArray {
    id: u32,
}

impl VertexArray {
    pub fn new<'a>() -> VertexArrayBuilder<'a> {
        VertexArrayBuilder {
            buffers: vec![],
            pointers: vec![],
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

pub struct VertexArrayBuilder<'a> {
    buffers: Vec<&'a Buffer>,
    pointers: Vec<&'a ArrayPointer>,
}

impl<'a> VertexArrayBuilder<'a> {
    pub fn buffer(mut self, buffer: &'a Buffer) -> Self {
        self.buffers.push(buffer);
        self
    }

    pub fn pointer(mut self, pointer: &'a ArrayPointer) -> Self {
        self.pointers.push(pointer);
        self
    }

    pub fn build(self) -> VertexArray {
        let id = unsafe {
            let mut id: u32 = mem::uninitialized();
            gl::GenVertexArrays(1, &mut id);
            gl::BindVertexArray(id);
            id
        };
        for buffer in self.buffers {
            buffer.bind();
        }
        for pointer in self.pointers {
            pointer.bind();
        }
        VertexArray::unbind();
        VertexArray { id }
    }
}

pub struct ArrayPointer {
    attribute: GLuint,
    components: GLint,
    data_type: GLenum,
    normalize: bool,
    stride: GLsizei,
    offset: GLsizei,
}

impl ArrayPointer {
    pub fn new() -> Self {
        Self {
            attribute: 0,
            components: 1,
            data_type: DataType::Float as GLenum,
            normalize: false,
            stride: 0,
            offset: 0,
        }
    }

    pub fn location(mut self, location: usize) -> Self {
        self.attribute = location as GLuint;
        self
    }

    pub fn shader_attribute(mut self, shader: &ShaderProgram, attribute: &str) -> Self {
        let attribute_c = CString::new(attribute).unwrap_or(CString::default());
        let attribute =
            unsafe { gl::GetAttribLocation(shader.id, attribute_c.as_ptr() as *const _) };
        self.attribute = attribute as GLuint;
        self
    }

    // Must be fewer than 5
    pub fn components(mut self, components: usize) -> Self {
        self.components = components as GLint;
        self
    }

    pub fn data_type(mut self, data_type: DataType) -> Self {
        self.data_type = data_type as GLenum;
        self
    }

    pub fn normalize(mut self, normalize: bool) -> Self {
        self.normalize = normalize;
        self
    }

    pub fn stride<T>(mut self, stride: usize) -> Self {
        self.stride = (stride * mem::size_of::<T>()) as GLsizei;
        self
    }

    pub fn offset<T>(mut self, offset: usize) -> Self {
        self.offset = (offset * mem::size_of::<T>()) as GLsizei;
        self
    }

    fn bind(&self) {
        unsafe {
            gl::VertexAttribPointer(
                self.attribute,
                self.components,
                self.data_type,
                self.normalize as u8,
                self.stride,
                self.offset as *const ffi::c_void,
            );
            gl::EnableVertexAttribArray(self.attribute);
        }
    }
}
