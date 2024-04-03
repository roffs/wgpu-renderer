mod camera;
mod graphics;
mod mesh;
mod texture;
mod vertex;

use camera::{Camera, CameraController};
use cgmath::{Deg, Vector3};
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

    let camera_controller = CameraController::new(10.0, 0.2);
    let mut camera = Camera::new(
        (0.0, 0.0, 3.0),
        Deg(-90.0),
        Deg(0.0),
        45.0,
        window.inner_size().width as f32 / window.inner_size().height as f32,
        0.01,
        100.0,
    );

    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop
        .run(|event, elwt| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                physical_key: PhysicalKey::Code(keycode),
                                ..
                            },
                        ..
                    } => {
                        match keycode {
                            KeyCode::Escape => elwt.exit(),
                            KeyCode::KeyW => camera_controller
                                .translate(&mut camera, Vector3::new(0.1, 0.0, 0.0)),
                            KeyCode::KeyS => camera_controller
                                .translate(&mut camera, Vector3::new(-0.1, 0.0, 0.0)),
                            KeyCode::Space => camera_controller
                                .translate(&mut camera, Vector3::new(0.0, 0.1, 0.0)),
                            KeyCode::ShiftLeft => camera_controller
                                .translate(&mut camera, Vector3::new(0.0, -0.1, 0.0)),
                            KeyCode::KeyA => camera_controller
                                .translate(&mut camera, Vector3::new(0.0, 0.0, -0.1)),
                            KeyCode::KeyD => camera_controller
                                .translate(&mut camera, Vector3::new(0.0, 0.0, 0.1)),
                            _ => {}
                        }
                    }
                    WindowEvent::RedrawRequested => {
                        graphics_context.render(&camera);
                    }
                    WindowEvent::Resized(size) => {
                        graphics_context.resize(size);
                    }
                    _ => {}
                }
            }

            Event::AboutToWait => window.request_redraw(),
            _ => {}
        })
        .unwrap();
}
