#![allow(dead_code)]
#![feature(map_entry_replace)]
#![feature(const_type_id)]

extern crate gl;
extern crate glutin;
extern crate image;
extern crate nalgebra_glm;
extern crate gltf;

mod rendering;
mod window;
mod data;

use image::GenericImageView;
use image::ImageFormat;
use nalgebra_glm as glm;
use rendering::{enumerations::*, *};
use std::time::SystemTime;
use window::Window;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut window = Window::new().title("Hello, world").build();

    let path_str = env::args_os().last().unwrap();
    let path = Path::new(&path_str);
    let crate_path = path.join("textures/crate.bmp");
    let face_path = path.join("textures/face.bmp");
    let vert_path = path.join("shaders/basic_vertex.glsl");
    let frag_path = path.join("shaders/basic_fragment.glsl");

    let shader = create_shader(vert_path.as_path(), frag_path.as_path());
    let vao = create_vao(&shader);
    let mesh = Mesh::new(&vao, data::INDICES.len() as i32);
    let crate_tex = create_texture(crate_path.as_path(), "tex1", &shader, 0);
    let face_tex = create_texture(face_path.as_path(), "tex2", &shader, 1);

    let mut time = UniformVector::new("time", &shader, vec![glm::Vec1::new(0.0)]).unwrap();
    let tx_model = UniformMatrix::new("model", &shader, model_matrix()).unwrap();

    let (width, height) = window.size();
    let camera = Camera::new()
        .rotation(0.0, -3.1415 / 5.0)
        .position(0.0, 1.0, 3.0)
        .viewport(width, height)
        .build();
    let mut camera_uniform = camera.to_uniform(&shader);

    let start_time = SystemTime::now();
    window.poll(|| {
        let elapsed = start_time.elapsed().unwrap().as_millis() as f32 / 1000.0f32;
        time.uniforms[0] = glm::Vec1::new(elapsed);
        time.set();

        crate_tex.activate(TextureUnit::_0);
        face_tex.activate(TextureUnit::_1);
        shader.bind();
        camera.update(&mut camera_uniform);

        globals::clear(1.0, 0.5, 0.7, 1.0, true);
        globals::test_depth(true);
        for i in 0..tx_model.uniforms.len() {
            tx_model.set_range(i, i + 1);
            mesh.draw();
        }
    });
}

fn model_matrix() -> Vec<glm::Mat4> {
    let mut planes = Vec::with_capacity(3);
    for i in 0..3 {
        let axis = glm::Vec3::new(
            (i == 0) as u8 as f32,
            (i == 1) as u8 as f32,
            (i == 2) as u8 as f32,
        );
        let mut rot: glm::Mat4 = glm::identity();
        rot = glm::rotate(&rot, 3.1415 / 4.0, &axis);
        planes.push(rot);
    }
    planes
}

fn create_texture(path: &Path, attribute: &str, shader: &ShaderProgram, unit: i32) -> Texture {
    let mut buffer: Vec<u8> = Vec::new();
    let mut file = File::open(path).unwrap();
    let _ = file.read_to_end(&mut buffer);
    let tex = texture_from_bmp(buffer.as_ref());
    let uniform = UniformVector::new(attribute, &shader, vec![glm::IVec1::new(unit)]).unwrap();
    uniform.set();
    tex
}

fn texture_from_bmp(data: &[u8]) -> Texture {
    let bmp = image::load_from_memory_with_format(data, ImageFormat::BMP).unwrap();
    let width = bmp.width() as usize;
    let height = bmp.height() as usize;
    let texture_data = bmp.to_rgb().into_raw();
    Texture::new(texture_data.as_ref(), width, height).build()
}

fn create_shader(vert_path: &Path, frag_path: &Path) -> ShaderProgram { 
    let mut buffer = String::new();
    let mut vert_file = File::open(vert_path).unwrap();
    let _ = vert_file.read_to_string(&mut buffer);
    let vert = Shader::vert(buffer.as_ref())
        .map_err(|err| println!("{}", err))
        .expect("Could not compile vertex shader.");
    buffer.clear();
    let mut frag_file = File::open(frag_path).unwrap();
    let _ = frag_file.read_to_string(&mut buffer);
    let frag = Shader::frag(buffer.as_ref())
        .map_err(|err| println!("{}", err))
        .expect("Could not compile fragment shader.");
    ShaderProgram::new()
        .with(&vert)
        .with(&frag)
        .build()
        .map_err(|err| println!("{}", err))
        .expect("Could not link shader program.")
}

fn create_vao(shader: &ShaderProgram) -> VertexArray {
    let vbo = Buffer::new(&data::VERTICES).build();
    let ebo = Buffer::new(&data::INDICES).kind(BufferKind::ElementArray).build();
    let positions = ArrayPointer::new()
        .shader_attribute(&shader, "position")
        .components(2)
        .stride::<f32>(4);
    let colors = ArrayPointer::new()
        .shader_attribute(&shader, "uv")
        .components(2)
        .stride::<f32>(4)
        .offset::<f32>(2);
    VertexArray::new()
        .buffer(&vbo)
        .buffer(&ebo)
        .pointer(&positions)
        .pointer(&colors)
        .build()
}