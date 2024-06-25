use std::path::Path;

use cgmath::{Deg, Vector3};
use wgpu::{Color, TextureView};
use winit::{
    event::{DeviceEvent, ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoopWindowTarget,
    keyboard::{KeyCode, PhysicalKey},
};

use crate::{
    camera::{Camera, CameraController},
    entity::{Entity, Geometry, Mesh, Node},
    gpu_context::GpuContext,
    layouts::Layouts,
    light::PointLight,
    material::Material,
    render_pass::{ModelPass, RenderPass, ShadowPass, SkyboxPass},
    resources::Resources,
    scene::Scene,
    skybox::Skybox,
    surface_context::SurfaceContext,
    transform::{Rotation, Transform},
};

pub struct App {
    _layouts: Layouts,
    camera_controller: CameraController,
    camera: Camera,
    scene: Scene,
    model_pass: ModelPass,
    skybox_pass: SkyboxPass,
    shadow_pass: ShadowPass,
}

impl App {
    pub fn new(context: &GpuContext, surface: &SurfaceContext) -> App {
        let GpuContext { device, queue, .. } = context;

        let layouts = Layouts::new(device);

        // CAMERA
        let camera_controller = CameraController::new(0.1, 0.1);
        let camera = Camera::new(
            (0.0, 2.0, 3.0),
            Deg(-90.0),
            Deg(0.0),
            45.0,
            surface.config().width as f32 / surface.config().height as f32,
            0.01,
            100.0,
        );

        // MODELS
        let helmet_transform = Transform::new(
            device,
            queue,
            &layouts.transform,
            (0.0, 1.0, 0.0),
            Some(Rotation::X(90.0)),
            1.0,
        );

        let helmet = Resources::load_gltf(
            device,
            queue,
            &layouts.material,
            Path::new("./assets/models/damaged_helmet/DamagedHelmet.gltf"),
        );

        let flat_cube_transform = Transform::new(
            device,
            queue,
            &layouts.transform,
            (3.0, 1.5, -2.0),
            Some(Rotation::X(-90.0)),
            2.0,
        );

        let flat_cube = Entity::new(
            vec![Node {
                mesh: Some(Mesh {
                    primitives: vec![(Geometry::cube(device), 0)],
                }),
                transform: None,
                children: Vec::new(),
            }],
            vec![Material::new(
                device,
                &layouts.material,
                Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
                Some(Resources::load_texture(
                    device,
                    queue,
                    Path::new("./assets/textures/test.png"),
                )),
                None,
            )],
        );

        let stone_cube_transform = Transform::new(
            device,
            queue,
            &layouts.transform,
            (-3.0, 1.5, 2.5),
            Some(Rotation::X(-90.0)),
            1.0,
        );

        let stone_cube = Resources::load_gltf(
            device,
            queue,
            &layouts.material,
            Path::new("./assets/models/stone_cube/scene.gltf"),
        );

        let shiba_transform = Transform::new(
            device,
            queue,
            &layouts.transform,
            (-2.0, 1.0, -2.0),
            Some(Rotation::X(-90.0)),
            1.0,
        );

        let shiba = Resources::load_gltf(
            device,
            queue,
            &layouts.material,
            Path::new("./assets/models/shiba/scene.gltf"),
        );

        let floor_transform = Transform::new(
            device,
            queue,
            &layouts.transform,
            (0.0, 0.0, 0.0),
            None,
            50.0,
        );

        let floor = Entity::new(
            vec![Node {
                mesh: Some(Mesh {
                    primitives: vec![(Geometry::plane(device), 0)],
                }),
                transform: None,
                children: Vec::new(),
            }],
            vec![Material::new(
                device,
                &layouts.material,
                Color::WHITE,
                None,
                None,
            )],
        );

        // LIGHT

        let light = PointLight::new(device, (7.5, 5.0, -4.0), (1.0, 0.0, 0.0));
        let second_light = PointLight::new(device, (-5.0, 4.0, 10.0), (0.0, 0.0, 1.0));
        let third_light = PointLight::new(device, (0.0, 5.0, 0.0), (1.0, 1.0, 1.0));

        // SKYBOX

        let skybox_paths = [
            Path::new("./assets/skybox/sky/right.jpg"),
            Path::new("./assets/skybox/sky/left.jpg"),
            Path::new("./assets/skybox/sky/top.jpg"),
            Path::new("./assets/skybox/sky/bottom.jpg"),
            Path::new("./assets/skybox/sky/front.jpg"),
            Path::new("./assets/skybox/sky/back.jpg"),
        ];

        let skybox_cubemap = Resources::load_cube_map(device, queue, skybox_paths);
        let skybox = Skybox::new(device, &layouts.skybox, &skybox_cubemap);

        // SCENE

        let entities = vec![
            (helmet, helmet_transform),
            (flat_cube, flat_cube_transform),
            (stone_cube, stone_cube_transform),
            (floor, floor_transform),
            (shiba, shiba_transform),
        ];

        let lights = vec![light, second_light, third_light];

        let scene = Scene {
            entities,
            lights,
            skybox,
        };

        let model_pass = ModelPass::new(device, surface.config(), &layouts, &scene.lights);
        let skybox_pass = SkyboxPass::new(device, surface.config(), &layouts);
        let shadow_pass = ShadowPass::new(device, surface.config(), &layouts, &scene.lights);

        App {
            _layouts: layouts,
            camera_controller,
            camera,
            scene,
            model_pass,
            skybox_pass,
            shadow_pass,
        }
    }

    pub fn update(&mut self, event: Event<()>, elwt: &EventLoopWindowTarget<()>) {
        match event {
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { event, .. },
                ..
            } => self.process_keyboard_event(event, elwt),
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => self.process_mouse_motion(delta),
            _ => {}
        }
    }

    fn process_mouse_motion(&mut self, delta: (f64, f64)) {
        self.camera_controller.yaw_delta += delta.0 as f32;
        self.camera_controller.pitch_delta += delta.1 as f32;
    }

    fn process_keyboard_event(&mut self, event: KeyEvent, elwt: &EventLoopWindowTarget<()>) {
        let KeyEvent {
            physical_key,
            state,
            repeat,
            ..
        } = event;

        if let (PhysicalKey::Code(keycode), false) = (physical_key, repeat) {
            match state {
                ElementState::Pressed => match keycode {
                    KeyCode::Escape => elwt.exit(),
                    KeyCode::KeyW => self.camera_controller.move_direction += Vector3::unit_x(),
                    KeyCode::KeyS => self.camera_controller.move_direction -= Vector3::unit_x(),
                    KeyCode::KeyA => self.camera_controller.move_direction -= Vector3::unit_z(),
                    KeyCode::KeyD => self.camera_controller.move_direction += Vector3::unit_z(),
                    KeyCode::Space => self.camera_controller.move_direction += Vector3::unit_y(),
                    KeyCode::ShiftLeft => {
                        self.camera_controller.move_direction -= Vector3::unit_y()
                    }
                    _ => {}
                },
                ElementState::Released => match keycode {
                    KeyCode::KeyW => self.camera_controller.move_direction -= Vector3::unit_x(),
                    KeyCode::KeyS => self.camera_controller.move_direction += Vector3::unit_x(),
                    KeyCode::KeyA => self.camera_controller.move_direction += Vector3::unit_z(),
                    KeyCode::KeyD => self.camera_controller.move_direction -= Vector3::unit_z(),
                    KeyCode::Space => self.camera_controller.move_direction -= Vector3::unit_y(),
                    KeyCode::ShiftLeft => {
                        self.camera_controller.move_direction += Vector3::unit_y()
                    }
                    _ => {}
                },
            }
        }
    }

    pub fn render(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, view: &TextureView) {
        self.camera_controller.update(&mut self.camera);

        self.shadow_pass
            .draw(device, queue, view, &self.camera, &self.scene);
        self.skybox_pass
            .draw(device, queue, view, &self.camera, &self.scene);
        self.model_pass
            .draw(device, queue, view, &self.camera, &self.scene);
    }

    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        self.camera.update_aspect(width as f32 / height as f32);

        self.skybox_pass.resize(device, width, height);
        self.model_pass.resize(device, width, height);
    }
}
