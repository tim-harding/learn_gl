extern crate gl;
extern crate glutin;
extern crate log;

mod rendering;

use glutin::{ContextBuilder, Event, EventsLoop, GlContext, GlWindow, WindowBuilder, WindowEvent};
use rendering::*;
use std::ffi::{CStr, CString};
use std::ptr;
use std::mem;

fn main() -> Result<(), ()> {
    let mut events_loop = EventsLoop::new();
    let window_builder = WindowBuilder::new().with_title("Hello World");
    let context_builder = ContextBuilder::new().with_vsync(true);
    let window = GlWindow::new(window_builder, context_builder, &events_loop).map_err(|_| ())?;

    unsafe {
        window.make_current().map_err(|_| ())?;
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    }

    let vertices: [f32; 6] = [
        -0.5, -0.5,
        0.0, 0.5,
        0.5, -0.5,
    ];
    let vert_source = include_str!("shaders/basic_vertex.glsl");
    let frag_source = include_str!("shaders/basic_fragment.glsl");
    let vert_source_c = CString::new(vert_source).map_err(|_| ())?;
    let frag_source_c = CString::new(frag_source).map_err(|_| ())?;
    let vert = Shader::vert(vert_source_c.as_ref())?;
    let frag = Shader::frag(frag_source_c.as_ref())?;
    let shader = ShaderProgram::new().with(&vert).with(&frag).build()?;
    shader.bind();

    let vbo = VertexBuffer::new(&vertices).build();
    let vao = VertexArray::new(vbo).components(2).build();
    vao.bind();

    let mut running = true;
    while running {
        events_loop.poll_events(|event| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => running = false,
                WindowEvent::Resized(logical_size) => {
                    let dpi_factor = window.get_hidpi_factor();
                    window.resize(logical_size.to_physical(dpi_factor));
                }
                _ => {}
            },
            _ => {}
        });

        unsafe {
            gl::ClearColor(1.0, 0.5, 0.7, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_buffers().expect("Could not swap backbuffer.");
    }
    Ok(())
}