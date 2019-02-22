use super::{ShaderProgram, UniformMatrix};
use nalgebra_glm as glm;

struct Rotation2D {
    horizontal: f32,
    tilt: f32,
}

impl Rotation2D {
    fn new(horizontal: f32, tilt: f32) -> Self {
        Self { horizontal, tilt }
    }

    fn default() -> Self {
        Self::new(0.0, 0.0)
    }

    fn apply(&self, matrix: &glm::Mat4) -> glm::Mat4 {
        let x_axis = glm::Vec3::new(1.0, 0.0, 0.0);
        let y_axis = glm::Vec3::new(0.0, 1.0, 0.0);
        let mut view = glm::rotate(matrix, -self.tilt, &x_axis);
        view = glm::rotate(&view, -self.horizontal, &y_axis);
        view
    }
}

pub struct Camera {
    position: glm::Vec3,
    rotation: Rotation2D,
    ratio: f32,
}

impl Camera {
    pub fn new() -> CameraBuilder {
        CameraBuilder {
            position: glm::Vec3::new(0.0, 0.0, 0.0),
            rotation: Rotation2D::default(),
            ratio: 1.0,
        }
    }

    pub fn update(&self, uniform: &mut UniformMatrix<glm::Mat4>) {
        uniform.uniforms[0] = self.view_projection();
        uniform.set_all();
    }

    pub fn to_uniform<'a>(&'a self, shader: &'a ShaderProgram) -> UniformMatrix<glm::Mat4> {
        UniformMatrix::new("view_proj", shader, vec![self.view_projection()]).unwrap()
    }

    fn view_projection(&self) -> glm::Mat4 {
        let proj = glm::perspective(3.1415 / 4.0, self.ratio, 0.1, 100.0);
        let position = -self.position;
        let mut view: glm::Mat4 = glm::identity();
        view = self.rotation.apply(&view);
        view = glm::translate(&view, &position);
        proj * view
    }
}

pub struct CameraBuilder {
    position: glm::Vec3,
    rotation: Rotation2D,
    ratio: f32,
}

impl CameraBuilder {
    pub fn build(self) -> Camera {
        Camera {
            position: self.position,
            rotation: self.rotation,
            ratio: self.ratio,
        }
    }

    pub fn position(mut self, x: f32, y: f32, z: f32) -> Self {
        self.position = glm::Vec3::new(x, y, z);
        self
    }

    pub fn rotation(mut self, horizontal: f32, tilt: f32) -> Self {
        self.rotation = Rotation2D { horizontal, tilt };
        self
    }

    pub fn viewport(mut self, width: f32, height: f32) -> Self {
        self.ratio = width / height;
        self
    }
}
