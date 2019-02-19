use super::{Material, ShaderProgram, Unary, UniformCacher};

pub struct BasicMaterial<'a> {
    shader: &'a ShaderProgram,
    scalar: UniformCacher<Unary>,
}

impl<'a> BasicMaterial<'a> {
    pub fn new(shader: &'a ShaderProgram) -> Self {
        Self {
            shader,
            scalar: UniformCacher::new(Unary::new(1.0), "scalar", shader),
        }
    }

    pub fn scalar(&mut self, scalar: f32) {
        self.scalar.uniform = Unary::new(scalar);
    }
}

impl<'a> Material for BasicMaterial<'a> {
    fn bind(&self) {
        self.scalar.set(self.shader);
        self.shader.bind();
    }
}
