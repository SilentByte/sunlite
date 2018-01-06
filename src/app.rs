//!
//! Game app and context initialization.
//!

use std::collections::VecDeque;

use glutin;
use glutin::GlContext;

use gfx;
use gfx::Device;
use gfx_device_gl;
use gfx_window_glutin;

type ColorFormat = gfx::format::Rgba8;
type DepthFormat = gfx::format::DepthStencil;

#[derive(Debug)]
pub enum Event {
    Idle,
    Close,
    Input,
    Update,
    Render,
}

pub struct App {
    gl_window: glutin::GlWindow,
    gl_loop: glutin::EventsLoop,
    gfx_device: gfx_device_gl::Device,
    gfx_factory: gfx_device_gl::Factory,
    gfx_main_fbo: gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat>,
    gfx_main_dbo: gfx::handle::DepthStencilView<gfx_device_gl::Resources, DepthFormat>,
    gfx_cmd_queue: gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
}

impl App {
    fn translate_native_event(&mut self, native_event: glutin::Event) -> Option<Event> {
        match native_event {
            glutin::Event::WindowEvent { event: we, .. } => {
                match we {
                    glutin::WindowEvent::Closed => Some(Event::Close),
                    _ => None
                }
            }
            _ => None
        }
    }

    pub fn run(&mut self) -> Option<Event> {
        let mut event_queue: VecDeque<glutin::Event> = VecDeque::new();
        self.gl_loop.poll_events(|e| {
            event_queue.push_back(e);
        });

        self.gfx_cmd_queue.clear(&self.gfx_main_fbo.clone(), [1.0, 0.78, 0.38, 0.0]);
        self.gfx_cmd_queue.clear_depth(&self.gfx_main_dbo.clone(), 1.0);

        self.gfx_cmd_queue.flush(&mut self.gfx_device);
        self.gl_window.swap_buffers();
        self.gfx_device.cleanup();

        while let Some(native_event) = event_queue.pop_front() {
            if let Some(e) = self.translate_native_event(native_event) {
                match e {
                    Event::Close => return None,
                    _ => return Some(e)
                }
            }
        }

        Some(Event::Idle)
    }
}

#[derive(Clone, Debug)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug)]
pub struct AppBuilder {
    title: String,
    size: Option<Size>,
    min_size: Option<Size>,
    max_size: Option<Size>,
    vsync: bool,
}

impl AppBuilder {
    pub fn new() -> Self {
        AppBuilder {
            title: "Sunlite".into(),
            size: None,
            max_size: None,
            min_size: None,
            vsync: false,
        }
    }

    pub fn build(&self) -> App {
        let events_loop = glutin::EventsLoop::new();
        let mut window_builder = glutin::WindowBuilder::new()
            .with_title(self.title.clone());

        if let Some(ref s) = self.size {
            window_builder = window_builder.with_dimensions(s.width, s.height);
        }

        if let Some(ref s) = self.min_size {
            window_builder = window_builder.with_min_dimensions(s.width, s.height);
        }

        if let Some(ref s) = self.max_size {
            window_builder = window_builder.with_max_dimensions(s.width, s.height);
        }

        let context_builder = glutin::ContextBuilder::new()
            .with_vsync(self.vsync);

        let (window, device, mut factory, main_fbo, main_dbo) =
            gfx_window_glutin::init::<ColorFormat, DepthFormat>
                (window_builder, context_builder, &events_loop);

        let cmd_queue: gfx::Encoder<_, _> = factory.create_command_buffer().into();

        App {
            gl_window: window,
            gl_loop: events_loop,
            gfx_device: device,
            gfx_factory: factory,
            gfx_main_fbo: main_fbo,
            gfx_main_dbo: main_dbo,
            gfx_cmd_queue: cmd_queue,
        }
    }

    pub fn title<S: Into<String>>(&mut self, title: S) -> &mut AppBuilder {
        self.title = title.into();
        self
    }

    pub fn size(&mut self, width: u32, height: u32) -> &mut AppBuilder {
        self.size = Some(Size { width, height });
        self
    }

    pub fn min_size(&mut self, width: u32, height: u32) -> &mut AppBuilder {
        self.min_size = Some(Size { width, height });
        self
    }

    pub fn max_size(&mut self, width: u32, height: u32) -> &mut AppBuilder {
        self.max_size = Some(Size { width, height });
        self
    }

    pub fn vsync(&mut self, vsync: bool) -> &mut AppBuilder {
        self.vsync = vsync;
        self
    }
}
