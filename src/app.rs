use std::path::Path;

use cgmath::{Deg, Quaternion, Rotation3, Vector3, Zero};
use wgpu::TextureView;
use winit::{
    event::{DeviceEvent, ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoopWindowTarget,
    keyboard::{KeyCode, PhysicalKey},
};

use crate::{
    camera::{Camera, CameraController},
    entity::{Entity, Geometry, Mesh, Node},
    environment_map::EnvironmentMap,
    gpu_context::GpuContext,
    layouts::Layouts,
    light::PointLight,
    material::Material,
    render_pass::{ModelPass, RenderPass, ShadowPass, SkyboxPass},
    render_world::RenderWorld,
    resources::Resources,
    scene::Scene,
    surface_context::SurfaceContext,
    texture::TextureType,
    transform::Transform,
};

pub struct App {
    layouts: Layouts,
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
        let helmet_transform = Transform::new((0.0, 1.0, 0.0), Quaternion::zero(), (1.0, 1.0, 1.0));

        let mut helmet = Resources::load_gltf(
            device,
            queue,
            Path::new("./assets/models/damaged_helmet/DamagedHelmet.gltf"),
        );

        helmet.apply_transform(helmet_transform);

        let flat_cube_transform = Transform::new(
            (3.0, 1.5, -2.0),
            Quaternion::from_angle_x(Deg(-90.0)),
            (2.0, 2.0, 2.0),
        );

        let flat_cube = Entity::new(
            vec![Node {
                mesh: Some(Mesh {
                    primitives: vec![(Geometry::cube(), 0)],
                }),
                transform: Transform::zero(),
                children: Vec::new(),
            }],
            vec![Material::new(
                [1.0, 1.0, 1.0, 1.0],
                Some(Resources::load_texture(
                    device,
                    queue,
                    Path::new("./assets/textures/test.png"),
                    TextureType::Diffuse,
                )),
                None,
                0.0,
                0.0,
                None,
                None,
            )],
            flat_cube_transform,
        );

        let stone_cube_transform = Transform::new(
            (-3.0, 1.5, 2.5),
            Quaternion::zero(),
            // (0.01, 0.01, 0.01),
            (1.0, 1.0, 1.0),
        );

        let mut stone_cube = Resources::load_gltf(
            device,
            queue,
            Path::new("./assets/models/stone_cube/scene.gltf"),
        );

        stone_cube.apply_transform(stone_cube_transform);

        let shiba_transform =
            Transform::new((-2.0, 1.0, -2.0), Quaternion::zero(), (1.0, 1.0, 1.0));

        let mut shiba =
            Resources::load_gltf(device, queue, Path::new("./assets/models/shiba/scene.gltf"));
        shiba.apply_transform(shiba_transform);

        let floor_transform =
            Transform::new((0.0, 0.0, 0.0), Quaternion::zero(), (25.0, 25.0, 25.0));

        let floor = Entity::new(
            vec![Node {
                mesh: Some(Mesh {
                    primitives: vec![(Geometry::plane(), 0)],
                }),
                transform: Transform::zero(),
                children: Vec::new(),
            }],
            vec![Material::new(
                [1.0, 1.0, 1.0, 1.0],
                Some(Resources::load_texture(
                    device,
                    queue,
                    Path::new("./assets/textures/brick-wall/brick-wall_albedo.png"),
                    TextureType::Diffuse,
                )),
                Some(Resources::load_texture(
                    device,
                    queue,
                    Path::new("./assets/textures/brick-wall/brick-wall_normal-ogl.png"),
                    TextureType::Normal,
                )),
                1.0,
                1.0,
                Some(Resources::load_texture(
                    device,
                    queue,
                    Path::new("./assets/textures/brick-wall/brick-wall_roughness.png"),
                    TextureType::Diffuse,
                )),
                Some(Resources::load_texture(
                    device,
                    queue,
                    Path::new("./assets/textures/brick-wall/brick-wall_ao.png"),
                    TextureType::Diffuse,
                )),
            )],
            floor_transform,
        );

        // LIGHT

        let light = PointLight::new((7.5, 5.0, -4.0), (1.0, 0.0, 0.0));
        let second_light = PointLight::new((-5.0, 4.0, 10.0), (0.0, 0.0, 1.0));
        let third_light = PointLight::new((0.0, 5.0, 0.0), (1.0, 1.0, 1.0));

        // SKYBOX

        let env_map_paths = [
            Path::new("./assets/skybox/sky/right.jpg"),
            Path::new("./assets/skybox/sky/left.jpg"),
            Path::new("./assets/skybox/sky/top.jpg"),
            Path::new("./assets/skybox/sky/bottom.jpg"),
            Path::new("./assets/skybox/sky/front.jpg"),
            Path::new("./assets/skybox/sky/back.jpg"),
        ];

        let env_map = EnvironmentMap::from(Resources::load_cube_map(device, queue, env_map_paths));

        // SCENE

        let entities = vec![helmet, flat_cube, stone_cube, floor, shiba];

        let lights = vec![light, second_light, third_light];

        let scene = Scene {
            entities,
            lights,
            env_map,
        };

        let model_pass = ModelPass::new(device, surface.config(), &layouts);
        let skybox_pass = SkyboxPass::new(device, surface.config(), &layouts);
        let shadow_pass = ShadowPass::new(device, surface.config(), &layouts);

        App {
            layouts,
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

        let render_world = RenderWorld::extract(
            device,
            queue,
            &self.layouts,
            &self.scene.entities,
            &self.camera,
            &self.scene.lights,
            &self.scene.env_map,
        );

        for light in render_world.lights.iter() {
            let shadow_map = &light.shadow_map;

            for (camera_index, camera) in light.shadow_cameras.iter().enumerate() {
                let shadow_map_view = &shadow_map.create_face_view(camera_index);
                self.shadow_pass
                    .draw(device, queue, shadow_map_view, &render_world, camera);
            }
        }

        self.skybox_pass
            .draw(device, queue, view, &render_world, &render_world.camera);
        self.model_pass
            .draw(device, queue, view, &render_world, &render_world.camera);
    }

    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        self.camera.update_aspect(width as f32 / height as f32);

        self.skybox_pass.resize(device, width, height);
        self.model_pass.resize(device, width, height);
    }
}
