#![allow(dead_code)]
#![feature(map_entry_replace)]
#![feature(const_type_id)]

extern crate gl;
extern crate glutin;
extern crate image;
extern crate nalgebra_glm;

mod rendering;
mod window;

use image::GenericImageView;
use image::ImageFormat;
use nalgebra_glm as glm;
use rendering::{enumerations::*, *};
use std::time::SystemTime;
use window::Window;

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

    let pairing = Mesh::new(&vao, indices.len() as i32);

    let crate_bmp = include_bytes!("textures/crate.bmp");
    let face_bmp = include_bytes!("textures/face.bmp");
    let crate_tex = texture_from_bmp(crate_bmp);
    let face_tex = texture_from_bmp(face_bmp);

    let tex1 = UniformVector::new("tex1", &shader, vec![glm::IVec1::new(0)]).unwrap();
    let tex2 = UniformVector::new("tex2", &shader, vec![glm::IVec1::new(1)]).unwrap();
    tex1.set();
    tex2.set();

    let mut time = UniformVector::new("time", &shader, vec![glm::Vec1::new(0.0)]).unwrap();
    time.set();

    let rot_2d: glm::Mat3 = glm::rotation2d(3.14 / 2.0);
    let mut rotation = UniformMatrix::new("transform", &shader, vec![rot_2d]).unwrap();
    rotation.set(false);

    let start_time = SystemTime::now();
    window.poll(|| {
        let elapsed = start_time.elapsed().unwrap().as_millis() as f32 / 1000.0f32;
        time.uniforms[0] = glm::Vec1::new(elapsed);
        time.set();

        rotation.uniforms[0] = glm::rotation2d(elapsed);
        rotation.set(false);

        crate_tex.activate(TextureUnit::_0);
        face_tex.activate(TextureUnit::_1);
        shader.bind();

        globals::clear(1.0, 0.5, 0.7, 1.0);
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
