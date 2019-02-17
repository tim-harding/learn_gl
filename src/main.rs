#![allow(dead_code)]

extern crate gl;
extern crate glutin;
extern crate log;

mod rendering;
mod window;

use rendering::*;
use std::time::SystemTime;
use window::Window;

fn main() {
    let mut window = Window::new().title("Hello, world").build();

    #[rustfmt::skip]
    let vertices: [f32; 20] = [
        0.5, 0.5, 1.0, 0.0, 0.0,
        0.5, -0.5, 0.0, 1.0, 0.0,
        -0.5, -0.5, 0.0, 0.0, 1.0,
        -0.5, 0.5, 0.0, 0.0, 0.0
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
        .stride::<f32>(5);
    let colors = ArrayPointer::new()
        .shader_attribute(&shader, "color")
        .components(3)
        .stride::<f32>(5)
        .offset::<f32>(2);
    let vao = VertexArray::new()
        .buffer(&vbo)
        .buffer(&ebo)
        .pointer(&positions)
        .pointer(&colors)
        .build();
    let pairing = Pairing::new(&shader, &vao, indices.len() as i32);

    let time = SystemTime::now();
    window.poll(|| {
        let elapsed = time.elapsed().unwrap().as_millis() as f32 / 1000.0f32;
        globals::set_uniform(elapsed, &shader, "time");
        globals::clear(1.0, 0.5, 0.7, 1.0);
        pairing.draw();
    });
}
