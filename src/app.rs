//!
//! Game app and context initialization.
//!

use std::collections::VecDeque;

use glutin;
use glutin::GlContext;
use glutin::GlWindow;
use glutin::EventsLoop as GlEventsLoop;
use glutin::Event as GlEvent;
use glutin::WindowEvent as GlWindowEvent;

pub enum Event {
    Idle,
    Close,
    Input,
    Update,
    Render,
}

pub struct App {
    gl_window: GlWindow,
    gl_loop: GlEventsLoop,
}

impl App {
    fn translate_native_event(&mut self, native_event: GlEvent) -> Option<Event> {
        match native_event {
            GlEvent::WindowEvent { event: we, .. } => {
                match we {
                    GlWindowEvent::Closed => Some(Event::Close),
                    _ => None
                }
            }
            _ => None
        }
    }

    pub fn run(&mut self) -> Option<Event> {
        let mut event_queue: VecDeque<GlEvent> = VecDeque::new();
        self.gl_loop.poll_events(|e| {
            event_queue.push_back(e);
        });

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
        let window_builder = glutin::WindowBuilder::new()
            .with_title(self.title.clone())
            .with_dimensions(self.window_size.width, self.window_size.height);

        let context_builder = glutin::ContextBuilder::new();

        let events_loop = glutin::EventsLoop::new();
        let window = glutin::GlWindow::new(window_builder, context_builder, &events_loop).unwrap();

        unsafe {
            window.make_current().expect("Failed to switch GL context to app.");
        };

        App {
            gl_window: window,
            gl_loop: events_loop,
        }
    }
}
