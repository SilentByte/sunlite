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

        self.gfx_cmd_queue.clear(&self.gfx_main_fbo.clone(), [1.0, 0.0, 0.0, 0.0]);
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
    window_size: Size,
}

impl AppBuilder {
    pub fn new() -> Self {
        AppBuilder {
            title: "Sunlite".into(),
            window_size: Size {
                width: 1200,
                height: 800,
            },
        }
    }

    pub fn build(&self) -> App {
        let events_loop = glutin::EventsLoop::new();
        let window_builder = glutin::WindowBuilder::new()
            .with_title(self.title.clone())
            .with_dimensions(self.window_size.width, self.window_size.height);

        let context_builder = glutin::ContextBuilder::new();
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
}
