mod camera;
mod gpu_context;
mod layouts;
mod light;
mod material;
mod model;
mod render_pass;
mod resources;
mod scene;
mod skybox;
mod surface_context;
mod texture;
mod transform;
mod window_context;

use std::path::Path;

use camera::{Camera, CameraController};
use cgmath::{Deg, Vector3};
use gpu_context::GpuContext;
use layouts::{Layout, Layouts};
use light::PointLight;
use material::Material;
use model::{Mesh, Model};
use render_pass::RenderPasses;
use resources::Resources;
use scene::Scene;
use skybox::Skybox;
use surface_context::SurfaceContext;
use transform::{Rotation, Transform};
use wgpu::Color;
use window_context::WindowContext;
use winit::{
    event::{DeviceEvent, ElementState, Event, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

fn main() {
    let window_loop = WindowContext::new();
    let mut surface = SurfaceContext::new();
    let context = GpuContext::new(&surface);
    surface.init(&context, window_loop.window.clone());

    let layouts = Layouts::new(&context.device);

    // CAMERA

    let mut camera_controller = CameraController::new(0.1, 0.1);
    let mut camera = Camera::new(
        (0.0, 1.0, 3.0),
        Deg(-90.0),
        Deg(0.0),
        45.0,
        surface.config().width as f32 / surface.config().height as f32,
        0.01,
        100.0,
    );

    //  MODELS
    let transform_matrix = Transform::new(
        &context.device,
        &context.queue,
        layouts.get(&Layout::Transform),
        (0.0, 1.0, 0.0),
        Some(Rotation::X(-90.0)),
        1.0,
    );

    let shiba = Resources::load_model(
        &context.device,
        &context.queue,
        layouts.get(&Layout::Material),
        Path::new("./assets/models/shiba/scene.gltf"),
    );

    let transform_matrix_2 = Transform::new(
        &context.device,
        &context.queue,
        layouts.get(&Layout::Transform),
        (3.0, 1.0, 0.0),
        Some(Rotation::X(-90.0)),
        1.0,
    );

    let flat_cube = Model {
        meshes: vec![(Mesh::cube(&context.device), 0)],
        materials: vec![Material::new(
            &context.device,
            layouts.get(&Layout::Material),
            Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            Some(Resources::load_texture(
                &context.device,
                &context.queue,
                Path::new("./assets/textures/test.png"),
            )),
            None,
        )],
    };

    let transform_matrix_3 = Transform::new(
        &context.device,
        &context.queue,
        layouts.get(&Layout::Transform),
        (-2.0, 1.0, 0.0),
        Some(Rotation::X(-90.0)),
        0.5,
    );

    let stone_cube = Resources::load_model(
        &context.device,
        &context.queue,
        layouts.get(&Layout::Material),
        Path::new("./assets/models/stone_cube/scene.gltf"),
    );

    let floor_transform = Transform::new(
        &context.device,
        &context.queue,
        layouts.get(&Layout::Transform),
        (0.0, 0.0, 0.0),
        None,
        10.0,
    );

    let floor = Model::new(
        vec![(Mesh::plane(&context.device), 0)],
        vec![Material::new(
            &context.device,
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

    let mut render_passes =
        RenderPasses::new(&context.device, &context.queue, surface.config(), &layouts);

    // SKYBOX

    let skybox_paths = [
        Path::new("./assets/skybox/sky/right.jpg"),
        Path::new("./assets/skybox/sky/left.jpg"),
        Path::new("./assets/skybox/sky/top.jpg"),
        Path::new("./assets/skybox/sky/bottom.jpg"),
        Path::new("./assets/skybox/sky/front.jpg"),
        Path::new("./assets/skybox/sky/back.jpg"),
    ];

    let skybox_cubemap = Resources::load_cube_map(&context.device, &context.queue, skybox_paths);
    let skybox = Skybox::new(
        &context.device,
        layouts.get(&Layout::Skybox),
        &skybox_cubemap,
    );

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

    window_loop
        .event_loop
        .run(|event, elwt| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window_loop.window.id() => match event {
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

                    let output = surface.get().unwrap().get_current_texture().unwrap();
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
                        surface.configure(&context.device, width, height);

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
            Event::AboutToWait => window_loop.window.request_redraw(),
            _ => {}
        })
        .unwrap();
}
