//!
//! Game app and context initialization.
//!

use glutin;
use glutin::GlContext;
use glutin::GlWindow;
use glutin::EventsLoop as GlEventsLoop;
use glutin::Event as GlEvent;
use glutin::WindowEvent as GlWindowEvent;

use std::collections::VecDeque;

pub struct App {
    gl_window: GlWindow,
    gl_loop: GlEventsLoop,
}

impl App {
    pub fn run(mut self) {
        let mut event_queue: VecDeque<GlEvent> = VecDeque::new();
        'main_loop: loop {
            self.gl_loop.poll_events(|e| {
                event_queue.push_back(e);
            });

            while let Some(e) = event_queue.pop_front() {
                if let GlEvent::WindowEvent {event: e, ..} = e {
                    match e {
                        GlWindowEvent::Closed => break 'main_loop,
                        _ => {}
                    }
                }
            }
        }
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
        let window_builder = glutin::WindowBuilder::new()
            .with_title("Sunlite")
            .with_dimensions(self.window_size.width, self.window_size.height);

        let context_builder = glutin::ContextBuilder::new();

        let events_loop = glutin::EventsLoop::new();
        let window = glutin::GlWindow::new(window_builder, context_builder, &events_loop).unwrap();

        unsafe {
            window.make_current();
        };

        App {
            gl_window: window,
            gl_loop: events_loop,
        }
    }
}
