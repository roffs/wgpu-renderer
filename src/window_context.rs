use std::sync::Arc;

use winit::{
    dpi::PhysicalSize,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub struct WindowContext {
    pub event_loop: EventLoop<()>,
    pub window: Arc<Window>,
}

impl WindowContext {
    pub fn new() -> WindowContext {
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new()
            .with_title("WGPU renderer")
            .with_inner_size(PhysicalSize {
                width: 1024,
                height: 768,
            })
            .build(&event_loop)
            .unwrap();

        window
            .set_cursor_grab(winit::window::CursorGrabMode::Confined)
            .unwrap();

        window.set_cursor_visible(false);

        event_loop.set_control_flow(ControlFlow::Poll);

        WindowContext {
            event_loop,
            window: Arc::new(window),
        }
    }
}
