mod camera;
mod graphics;
mod mesh;
mod texture;
mod vertex;

use camera::{Camera, CameraController};
use cgmath::{Deg, InnerSpace, Vector3, Zero};
use graphics::GraphicsContext;
use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
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

    let camera_controller = CameraController::new(0.1, 0.2);
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

    let mut translation_direction = Vector3::<f32>::zero();

    event_loop
        .run(|event, elwt| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: PhysicalKey::Code(keycode),
                            state,
                            repeat: false,
                            ..
                        },
                    ..
                } => match state {
                    ElementState::Pressed => match keycode {
                        KeyCode::Escape => elwt.exit(),
                        KeyCode::KeyW => translation_direction += Vector3::unit_x(),
                        KeyCode::KeyS => translation_direction -= Vector3::unit_x(),
                        KeyCode::KeyA => translation_direction -= Vector3::unit_z(),
                        KeyCode::KeyD => translation_direction += Vector3::unit_z(),
                        KeyCode::Space => translation_direction += Vector3::unit_y(),
                        KeyCode::ShiftLeft => translation_direction -= Vector3::unit_y(),
                        _ => {}
                    },
                    ElementState::Released => match keycode {
                        KeyCode::KeyW => translation_direction -= Vector3::unit_x(),
                        KeyCode::KeyS => translation_direction += Vector3::unit_x(),
                        KeyCode::KeyA => translation_direction += Vector3::unit_z(),
                        KeyCode::KeyD => translation_direction -= Vector3::unit_z(),
                        KeyCode::Space => translation_direction -= Vector3::unit_y(),
                        KeyCode::ShiftLeft => translation_direction += Vector3::unit_y(),
                        _ => {}
                    },
                },
                WindowEvent::RedrawRequested => {
                    camera_controller.translate(&mut camera, translation_direction);
                    graphics_context.render(&camera);
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
