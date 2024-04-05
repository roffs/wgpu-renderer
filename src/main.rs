mod camera;
mod material;
mod model;
mod render_pass;
mod resources;
mod texture;
mod transform;

use camera::{Camera, CameraController, CameraDescriptor};
use cgmath::{Deg, Vector3, Zero};
use render_pass::RenderPass;
use resources::Resources;
use transform::Transform;
use wgpu::{
    CompositeAlphaMode, Device, DeviceDescriptor, Features, Instance, InstanceDescriptor, Limits,
    Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, TextureUsages,
};
use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

fn main() {
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

    let (device, queue, mut config, surface) = create_graphics_context(&window);

    let resources = Resources::new(&device, &queue);

    // CAMERA

    let camera_controller = CameraController::new(0.1, 0.1);
    let mut camera = Camera::new(
        &device,
        &queue,
        &resources.transform_bind_group_layout,
        CameraDescriptor {
            position: (0.0, 0.0, 3.0),
            yaw: Deg(-90.0),
            pitch: Deg(0.0),
            fovy: 45.0,
            aspect: window.inner_size().width as f32 / window.inner_size().height as f32,
            near: 0.01,
            far: 100.0,
        },
    );

    let mut renderer = RenderPass::new(&device, &queue, &surface, &mut config, &resources);

    // GROUP THINS INTO A MODEL
    let transform_matrix = Transform::new(
        &device,
        &queue,
        &resources.transform_bind_group_layout,
        (0.0, 0.0, 0.0),
        1.0,
    );

    let shiba = resources.load_model("./assets/models/shiba/scene.gltf");

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut camera_translation_direction = Vector3::<f32>::zero();
    let mut camera_delta_pitch = 0.0;
    let mut camera_delta_yaw = 0.0;

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
                        KeyCode::KeyW => camera_translation_direction += Vector3::unit_x(),
                        KeyCode::KeyS => camera_translation_direction -= Vector3::unit_x(),
                        KeyCode::KeyA => camera_translation_direction -= Vector3::unit_z(),
                        KeyCode::KeyD => camera_translation_direction += Vector3::unit_z(),
                        KeyCode::Space => camera_translation_direction += Vector3::unit_y(),
                        KeyCode::ShiftLeft => camera_translation_direction -= Vector3::unit_y(),
                        _ => {}
                    },
                    ElementState::Released => match keycode {
                        KeyCode::KeyW => camera_translation_direction -= Vector3::unit_x(),
                        KeyCode::KeyS => camera_translation_direction += Vector3::unit_x(),
                        KeyCode::KeyA => camera_translation_direction += Vector3::unit_z(),
                        KeyCode::KeyD => camera_translation_direction -= Vector3::unit_z(),
                        KeyCode::Space => camera_translation_direction -= Vector3::unit_y(),
                        KeyCode::ShiftLeft => camera_translation_direction += Vector3::unit_y(),
                        _ => {}
                    },
                },
                WindowEvent::RedrawRequested => {
                    camera_controller.translate(&mut camera, camera_translation_direction);
                    camera_controller.rotate(
                        &mut camera,
                        (camera_delta_pitch as f32, camera_delta_yaw as f32),
                    );
                    camera_delta_pitch = 0.0;
                    camera_delta_yaw = 0.0;

                    let objects = [(&shiba, &transform_matrix)];

                    renderer.render(&objects, &camera);
                }
                WindowEvent::Resized(size) => {
                    renderer.resize(size.width, size.height);
                    camera.update_aspect(size.width as f32 / size.height as f32);
                }
                _ => {}
            },
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                camera_delta_pitch += delta.0;
                camera_delta_yaw += delta.1;
            }
            Event::AboutToWait => window.request_redraw(),
            _ => {}
        })
        .unwrap();
}

fn create_graphics_context(window: &Window) -> (Device, Queue, SurfaceConfiguration, Surface) {
    let instance = Instance::new(InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let surface = instance.create_surface(window).unwrap();

    let adapter = pollster::block_on(async {
        instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
    })
    .unwrap();

    let (device, queue) = pollster::block_on(async {
        adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("Device"),
                    required_features: Features::empty(),
                    required_limits: Limits::default(),
                },
                None,
            )
            .await
    })
    .unwrap();

    let size = window.inner_size();

    let config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        desired_maximum_frame_latency: 2,
        alpha_mode: CompositeAlphaMode::Auto,
        view_formats: vec![],
    };

    surface.configure(&device, &config);

    (device, queue, config, surface)
}
