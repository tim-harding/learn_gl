#![allow(dead_code)]
#![feature(map_entry_replace)]

extern crate gl;
extern crate glutin;
extern crate image;
extern crate nalgebra_glm;

mod rendering;
mod window;

use image::GenericImageView;
use image::ImageFormat;
use rendering::{enumerations::*, *};
use std::time::SystemTime;
use window::Window;
use nalgebra_glm::{IVec1, Vec1};

fn main() {
    let mut window = Window::new().title("Hello, world").build();

    #[rustfmt::skip]
    let vertices: [f32; 16] = [
        0.5, 0.5, 1.0, 1.0,
        0.5, -0.5, 1.0, 0.0,
        -0.5, -0.5, 0.0, 0.0,
        -0.5, 0.5, 0.0, 1.0,
    ];
    #[rustfmt::skip]
    let indices: [u32; 6] = [
        0, 1, 3,
        1, 2, 3
    ];
    let vert_source = include_str!("shaders/basic_vertex.glsl");
    let frag_source = include_str!("shaders/basic_fragment.glsl");
    let vert = Shader::vert(vert_source)
        .map_err(|err| println!("{}", err))
        .expect("Could not compile vertex shader.");
    let frag = Shader::frag(frag_source)
        .map_err(|err| println!("{}", err))
        .expect("Could not compile fragment shader.");
    let shader = ShaderProgram::new()
        .with(&vert)
        .with(&frag)
        .build()
        .map_err(|err| println!("{}", err))
        .expect("Could not link shader program.");

    let vbo = Buffer::new(&vertices).build();
    let ebo = Buffer::new(&indices).kind(BufferKind::ElementArray).build();
    let positions = ArrayPointer::new()
        .shader_attribute(&shader, "position")
        .components(2)
        .stride::<f32>(4);
    let colors = ArrayPointer::new()
        .shader_attribute(&shader, "uv")
        .components(2)
        .stride::<f32>(4)
        .offset::<f32>(2);
    let vao = VertexArray::new()
        .buffer(&vbo)
        .buffer(&ebo)
        .pointer(&positions)
        .pointer(&colors)
        .build();

    let crate_bmp = include_bytes!("textures/crate.bmp");
    let face_bmp = include_bytes!("textures/face.bmp");
    let crate_tex = texture_from_bmp(crate_bmp);
    let face_tex = texture_from_bmp(face_bmp);

    let tex1 = Uniform::new("tex1", &shader);
    IVec1::new(0).set(&tex1);
    let tex2 = Uniform::new("tex2", &shader);
    IVec1::new(1).set(&tex2);

    let pairing = Mesh::new(&vao, indices.len() as i32);
    let time_uniform = Uniform::new("time", &shader);

    let time = SystemTime::now();
    window.poll(|| {
        let elapsed = time.elapsed().unwrap().as_millis() as f32 / 1000.0f32;
        Vec1::new(elapsed).set(&time_uniform);
        globals::clear(1.0, 0.5, 0.7, 1.0);
        crate_tex.activate(TextureUnit::_0);
        face_tex.activate(TextureUnit::_1);
        shader.bind();
        pairing.draw();
    });
}

fn texture_from_bmp(data: &[u8]) -> Texture {
    let bmp = image::load_from_memory_with_format(data, ImageFormat::BMP).unwrap();
    let width = bmp.width() as usize;
    let height = bmp.height() as usize;
    let texture_data = bmp.to_rgb().into_raw();
    Texture::new(texture_data.as_ref(), width, height).build()
}
