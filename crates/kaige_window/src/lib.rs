// This will be integrated into the rendering crate, will be deprecated.
// Leaving this here for now just in case I need to use it for something.
// This is just a placeholder for now, will be removed later.
// TODO: Remove this file.


use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct App {
    event_loop: EventLoop<()>,
}

impl App {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();

        Self { event_loop }
    }

    pub fn run(self) {
        let _window = WindowBuilder::new().build(&self.event_loop).unwrap();

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => (),
            }
        });
    }
}
