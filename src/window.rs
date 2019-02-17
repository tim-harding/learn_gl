use glutin::{dpi::LogicalSize, ContextBuilder, Event, EventsLoop, GlContext, WindowEvent};

pub struct Window {
    events_loop: glutin::EventsLoop,
    window: glutin::GlWindow,
}

impl Window {
    pub fn new<'a>() -> WindowBuilder<'a> {
        WindowBuilder { title: "" }
    }

    pub fn poll<T>(&mut self, mut callback: T)
    where
        T: FnMut() -> (),
    {
        let mut running = true;
        let mut size: (bool, LogicalSize) = (false, LogicalSize::new(0.0, 0.0));
        while running {
            self.events_loop.poll_events(|event| match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => running = false,
                    WindowEvent::Resized(logical_size) => size = (true, logical_size),
                    _ => {}
                },
                _ => {}
            });
            if size.0 {
                size.0 = false;
                self.resize(size.1);
            }
            callback();
            self.swap();
        }
    }

    pub fn resize(&self, size: glutin::dpi::LogicalSize) {
        let dpi_factor = self.window.get_hidpi_factor();
        self.window.resize(size.to_physical(dpi_factor));
    }

    pub fn swap(&mut self) {
        self.window
            .swap_buffers()
            .expect("Could not swap backbuffer.");
    }
}

pub struct WindowBuilder<'a> {
    title: &'a str,
}

impl<'a> WindowBuilder<'a> {
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }

    pub fn build(self) -> Window {
        let events_loop = EventsLoop::new();
        let window_builder = glutin::WindowBuilder::new().with_title(self.title);
        let context_builder = ContextBuilder::new().with_vsync(true);
        let window = glutin::GlWindow::new(window_builder, context_builder, &events_loop)
            .expect("Could not open a window.");

        unsafe {
            window
                .make_current()
                .expect("Could not make window current.");
            gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        }
        Window {
            events_loop,
            window,
        }
    }
}
