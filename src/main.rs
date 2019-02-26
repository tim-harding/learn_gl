#![allow(dead_code)]
#![feature(map_entry_replace)]
#![feature(const_type_id)]

extern crate gl;
extern crate gltf;
extern crate glutin;
extern crate image;
extern crate nalgebra_glm;

mod data;
mod rendering;
mod window;

use gltf::{mesh::util::ReadIndices::*};
use image::GenericImageView;
use image::ImageFormat;
use nalgebra_glm as glm;
use rendering::{enumerations::*, *};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use window::Window;

fn main() {
    let mut window = Window::new().title("Hello, world").build();

    let shader = create_shader("shaders/basic_vertex.glsl", "shaders/basic_fragment.glsl");

    let (contents, buffers, _) =
        gltf::import(full_path("models/the_utah_teapot/scene.gltf")).unwrap();
    let meshes = contents.meshes().last().unwrap();
    let prim = meshes.primitives().last().unwrap();
    let reader = prim.reader(|buffer| Some(&buffers[buffer.index()]));
    let positions: Vec<_> = reader.read_positions().unwrap().collect();
    let indices: Vec<_> = match reader.read_indices().unwrap() {
        U32(iter) => iter.collect(),
        _ => {
            println!("Wrong indices type");
            return;
        }
    };
    let vbo = Buffer::new(&positions).build();
    let ebo = Buffer::new(&indices).kind(BufferKind::ElementArray).build();
    let pos_ptr = ArrayPointer::new()
        .shader_attribute(&shader, "position")
        .components(3);
    let vao = VertexArray::new()
        .buffer(&vbo)
        .buffer(&ebo)
        .pointer(&pos_ptr)
        .build();
    let mesh = Mesh::new(&vao, indices.len() as i32);

    let (width, height) = window.size();
    let camera = Camera::new()
        .viewport(width, height)
        .build();
    let mut camera_uniform = camera.to_uniform(&shader);

    window.poll(|| {
        shader.bind();
        camera.update(&mut camera_uniform);

        globals::clear(1.0, 0.5, 0.7, 1.0, true);
        globals::test_depth(true);
        mesh.draw();
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

fn create_texture(path: &str, attribute: &str, shader: &ShaderProgram, unit: i32) -> Texture {
    let mut buffer: Vec<u8> = Vec::new();
    let mut file = File::open(full_path(path)).unwrap();
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

fn create_shader(vert_path: &str, frag_path: &str) -> ShaderProgram {
    let mut buffer = String::new();
    let mut vert_file = File::open(full_path(vert_path)).unwrap();
    let _ = vert_file.read_to_string(&mut buffer);
    let vert = Shader::vert(buffer.as_ref())
        .map_err(|err| println!("{}", err))
        .expect("Could not compile vertex shader.");
    buffer.clear();
    let mut frag_file = File::open(full_path(frag_path)).unwrap();
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
    let ebo = Buffer::new(&data::INDICES)
        .kind(BufferKind::ElementArray)
        .build();
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

fn full_path(sub_path: &str) -> Box<Path> {
    let path_str = env::args_os().last().unwrap();
    let path = Path::new(&path_str);
    path.join(sub_path).into_boxed_path()
}
