extern crate gl;
extern crate glutin;
extern crate log;

mod rendering;

use glutin::{ContextBuilder, Event, EventsLoop, GlContext, GlWindow, WindowBuilder, WindowEvent};
use rendering::*;
use std::ffi::{CStr, CString};

fn main() -> Result<(), ()> {
    let mut events_loop = EventsLoop::new();
    let window_builder = WindowBuilder::new().with_title("Hello World");
    let context_builder = ContextBuilder::new().with_vsync(true);
    let window = GlWindow::new(window_builder, context_builder, &events_loop).map_err(|_| ())?;

    unsafe {
        window.make_current().map_err(|_| ())?;
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    }

    let vertices = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
    let vbo = VertexBuffer::new(&vertices).build();
    let vao = VertexArray::new(vbo).components(3).build();

    let vertex_shader_source = CString::new(include_str!("shaders/basic_vertex.glsl"));
    let fragment_shader_source = CString::new(include_str!("shaders/basic_fragment.glsl"));
    let vertex_shader = Shader::vert(vertex_shader_source.as_ref().unwrap())?;
    let fragment_shader = Shader::frag(fragment_shader_source.as_ref().unwrap())?;
    let shader = ShaderProgram::new()
        .with(vertex_shader)
        .with(fragment_shader)
        .build()?;

    let version = unsafe {
        let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _).to_bytes().to_vec();
        String::from_utf8(data).unwrap()
    };

    println!("OpenGL version {}", version);

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

        vao.bind();
        shader.bind();

        unsafe {
            gl::ClearColor(0.0, 1.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_buffers().expect("Could not swap backbuffer.");
    }
    Ok(())
}
