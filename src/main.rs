mod graphics;
mod texture;
mod vertex;

use graphics::GraphicsContext;
use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("WGPU renderer")
        .build(&event_loop)
        .unwrap();

    let mut graphics_context = GraphicsContext::new(&window);

    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop
        .run(|event, elwt| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            ..
                        },
                    ..
                } => elwt.exit(),
                WindowEvent::RedrawRequested => {
                    graphics_context.render();
                }
                WindowEvent::Resized(size) => {
                    graphics_context.resize(size);
                }
                _ => {}
            },

            Event::AboutToWait => window.request_redraw(),
            _ => {}
        })
        .unwrap();
}
