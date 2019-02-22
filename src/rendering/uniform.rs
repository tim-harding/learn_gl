use super::ShaderProgram;
use gl::types::*;
use std::any::TypeId;
use nalgebra_glm::*;

type Assignment<T> = unsafe fn(GLuint, GLint, GLsizei, *const T);

#[derive(Debug)]
enum Kind {Float, Int, Uint}

const ASSIGNMENT_F: [Assignment<GLfloat>; 4] = [
    gl::ProgramUniform1fv,
    gl::ProgramUniform2fv,
    gl::ProgramUniform3fv,
    gl::ProgramUniform4fv,
];
const ASSIGNMENT_I: [Assignment<GLint>; 4] = [
    gl::ProgramUniform1iv,
    gl::ProgramUniform2iv,
    gl::ProgramUniform3iv,
    gl::ProgramUniform4iv,
];
const ASSIGNMENT_U: [Assignment<GLuint>; 4] = [
    gl::ProgramUniform1uiv,
    gl::ProgramUniform2uiv,
    gl::ProgramUniform3uiv,
    gl::ProgramUniform4uiv,
];
const IDS_F: [TypeId; 4] = [
    TypeId::of::<Vec1>(),
    TypeId::of::<Vec2>(),
    TypeId::of::<Vec3>(),
    TypeId::of::<Vec4>(),
];
const IDS_I: [TypeId; 4] = [
    TypeId::of::<IVec1>(),
    TypeId::of::<IVec2>(),
    TypeId::of::<IVec3>(),
    TypeId::of::<IVec4>(),
];
const IDS_U: [TypeId; 4] = [
    TypeId::of::<U32Vec1>(),
    TypeId::of::<U32Vec2>(),
    TypeId::of::<U32Vec3>(),
    TypeId::of::<U32Vec4>(),
];

pub struct UniformVector<'a, T>
where
    T: 'static,
{
    shader: &'a ShaderProgram,
    location: GLint,
    pub uniforms: Vec<T>,
    kind: Kind,
    components: usize,
}

impl<'a, T> UniformVector<'a, T>
where
    T: 'static,
{
    pub fn set(&self) {
        match &self.kind {
            Kind::Float => self.assign(ASSIGNMENT_F),
            Kind::Int => self.assign(ASSIGNMENT_I),
            Kind::Uint => self.assign(ASSIGNMENT_U),
        };
    }

    pub fn new(attribute: &str, shader: &'a ShaderProgram, uniforms: Vec<T>) -> Option<Self> {
        let vector_id = TypeId::of::<T>();
        let location = shader.location(attribute);
        Self::kind_components::<T>()
            .map(|(kind, components)| Self{shader, location, uniforms, kind, components})
    }

    fn kind_components<U: 'static>() -> Option<(Kind, usize)> {
        let id = TypeId::of::<U>();
        let id_arrays = [IDS_F, IDS_I, IDS_U];
        for (i, array) in id_arrays.iter().enumerate() {
            if let Some(components) = Self::index_of(id, array) {
                let kind = match i {
                    0 => Kind::Float,
                    1 => Kind::Int,
                    _ => Kind::Uint,
                };
                return Some((kind, components));
            }
        };
        None
    }

    fn index_of(id: TypeId, array: &[TypeId]) -> Option<usize> {
        array.iter().position(|item| *item == id)
    }

    fn assign<U>(&self, array: [Assignment<U>; 4]) {
        let count = self.uniforms.len() as i32;
        let uniforms_ptr = self.uniforms.as_ptr() as *const U;
        let assignment = array[self.components];
        unsafe {
            assignment(self.shader.id, self.location, count, uniforms_ptr);
        }
    }
}