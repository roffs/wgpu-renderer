mod camera;
mod layouts;
mod light;
mod material;
mod model;
mod render_pass;
mod resources;
mod scene;
mod skybox;
mod texture;
mod transform;

use std::path::Path;

use camera::{Camera, CameraController, CameraDescriptor};
use cgmath::{Deg, Vector3};
use layouts::{Layout, Layouts};
use light::PointLight;
use material::Material;
use model::{Mesh, Model};
use render_pass::RenderPasses;
use resources::Resources;
use scene::Scene;
use skybox::Skybox;
use transform::{Rotation, Transform};
use wgpu::{
    Color, CompositeAlphaMode, Device, DeviceDescriptor, Features, Instance, InstanceDescriptor,
    Limits, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, TextureUsages,
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

    let layouts = Layouts::new(&device);

    // CAMERA

    let mut camera_controller = CameraController::new(0.1, 0.1);
    let mut camera = Camera::new(CameraDescriptor {
        position: (0.0, 1.0, 3.0),
        yaw: Deg(-90.0),
        pitch: Deg(0.0),
        fovy: 45.0,
        aspect: window.inner_size().width as f32 / window.inner_size().height as f32,
        near: 0.01,
        far: 100.0,
    });

    //  MODELS
    let transform_matrix = Transform::new(
        &device,
        &queue,
        layouts.get(&Layout::Transform),
        (0.0, 1.0, 0.0),
        Some(Rotation::X(-90.0)),
        1.0,
    );

    let shiba = Resources::load_model(
        &device,
        &queue,
        layouts.get(&Layout::Material),
        Path::new("./assets/models/shiba/scene.gltf"),
    );

    let transform_matrix_2 = Transform::new(
        &device,
        &queue,
        layouts.get(&Layout::Transform),
        (3.0, 1.0, 0.0),
        Some(Rotation::X(-90.0)),
        1.0,
    );

    let flat_cube = Model {
        meshes: vec![(Mesh::cube(&device), 0)],
        materials: vec![Material::new(
            &device,
            layouts.get(&Layout::Material),
            Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            Some(Resources::load_texture(
                &device,
                &queue,
                Path::new("./assets/textures/test.png"),
            )),
            None,
        )],
    };

    let transform_matrix_3 = Transform::new(
        &device,
        &queue,
        layouts.get(&Layout::Transform),
        (-2.0, 1.0, 0.0),
        Some(Rotation::X(-90.0)),
        0.5,
    );

    let stone_cube = Resources::load_model(
        &device,
        &queue,
        layouts.get(&Layout::Material),
        Path::new("./assets/models/stone_cube/scene.gltf"),
    );

    let floor_transform = Transform::new(
        &device,
        &queue,
        layouts.get(&Layout::Transform),
        (0.0, 0.0, 0.0),
        None,
        10.0,
    );

    let floor = Model::new(
        vec![(Mesh::plane(&device), 0)],
        vec![Material::new(
            &device,
            layouts.get(&Layout::Material),
            Color {
                r: 1.0,
                g: 0.4,
                b: 0.2,
                a: 1.0,
            },
            None,
            None,
        )],
    );

    // MODEL RENDER PASS

    let mut render_passes = RenderPasses::new(&device, &queue, &config, &layouts);

    // SKYBOX

    let skybox_paths = [
        Path::new("./assets/skybox/sky/right.jpg"),
        Path::new("./assets/skybox/sky/left.jpg"),
        Path::new("./assets/skybox/sky/top.jpg"),
        Path::new("./assets/skybox/sky/bottom.jpg"),
        Path::new("./assets/skybox/sky/front.jpg"),
        Path::new("./assets/skybox/sky/back.jpg"),
    ];

    let skybox_cubemap = Resources::load_cube_map(&device, &queue, skybox_paths);
    let skybox = Skybox::new(&device, layouts.get(&Layout::Skybox), &skybox_cubemap);

    // LIGHT

    let light = PointLight::new((1.0, 5.0, 0.0), (1.0, 1.0, 1.0));
    let second_light = PointLight::new((-1.0, 1.0, 1.0), (1.0, 1.0, 1.0));

    let lights = vec![light, second_light];

    // SCENE
    let entities = vec![
        (shiba, transform_matrix),
        (flat_cube, transform_matrix_2),
        (stone_cube, transform_matrix_3),
        (floor, floor_transform),
    ];

    let scene = Scene {
        entities,
        lights,
        skybox,
    };

    event_loop.set_control_flow(ControlFlow::Poll);

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
                        KeyCode::KeyW => camera_controller.move_direction += Vector3::unit_x(),
                        KeyCode::KeyS => camera_controller.move_direction -= Vector3::unit_x(),
                        KeyCode::KeyA => camera_controller.move_direction -= Vector3::unit_z(),
                        KeyCode::KeyD => camera_controller.move_direction += Vector3::unit_z(),
                        KeyCode::Space => camera_controller.move_direction += Vector3::unit_y(),
                        KeyCode::ShiftLeft => camera_controller.move_direction -= Vector3::unit_y(),
                        _ => {}
                    },
                    ElementState::Released => match keycode {
                        KeyCode::KeyW => camera_controller.move_direction -= Vector3::unit_x(),
                        KeyCode::KeyS => camera_controller.move_direction += Vector3::unit_x(),
                        KeyCode::KeyA => camera_controller.move_direction += Vector3::unit_z(),
                        KeyCode::KeyD => camera_controller.move_direction -= Vector3::unit_z(),
                        KeyCode::Space => camera_controller.move_direction -= Vector3::unit_y(),
                        KeyCode::ShiftLeft => camera_controller.move_direction += Vector3::unit_y(),
                        _ => {}
                    },
                },
                WindowEvent::RedrawRequested => {
                    camera_controller.update(&mut camera);

                    let output = surface.get_current_texture().unwrap();
                    let view = output
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());

                    let model_pass = render_passes.get(&render_pass::PassKind::Model);
                    let skybox_pass = render_passes.get(&render_pass::PassKind::Skybox);

                    skybox_pass.draw(&view, &camera, &scene);
                    model_pass.draw(&view, &camera, &scene);

                    output.present();
                }
                WindowEvent::Resized(size) => {
                    let width = size.width;
                    let height = size.height;

                    if width > 0 && height > 0 {
                        config.width = width;
                        config.height = height;

                        surface.configure(&device, &config);

                        camera.update_aspect(width as f32 / height as f32);
                        render_passes.resize(width, height)
                    }
                }
                _ => {}
            },
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                camera_controller.yaw_delta += delta.0 as f32;
                camera_controller.pitch_delta += delta.1 as f32;
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
