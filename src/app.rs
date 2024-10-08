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
    gpu_context::GpuContext,
    layouts::Layouts,
    light::PointLight,
    material::Material,
    render_pass::{HdrPipeline, PbrPass, ShadowPass, SkyboxPass},
    render_world::RenderWorld,
    resources::{Resources, SkyboxLoader},
    scene::Scene,
    surface_context::SurfaceContext,
    texture::Texture,
    transform::Transform,
};

pub struct App {
    layouts: Layouts,
    camera_controller: CameraController,
    camera: Camera,
    scene: Scene,
    model_pass: PbrPass,
    skybox_pass: SkyboxPass,
    shadow_pass: ShadowPass,
    hdr_pipeline: HdrPipeline,
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
                    Texture::SRGBA_UNORM,
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
                    Texture::SRGBA_UNORM,
                )),
                Some(Resources::load_texture(
                    device,
                    queue,
                    Path::new("./assets/textures/brick-wall/brick-wall_normal-ogl.png"),
                    Texture::RGBA_UNORM,
                )),
                1.0,
                1.0,
                Some(Resources::load_texture(
                    device,
                    queue,
                    Path::new("./assets/textures/brick-wall/brick-wall_roughness.png"),
                    Texture::SRGBA_UNORM,
                )),
                Some(Resources::load_texture(
                    device,
                    queue,
                    Path::new("./assets/textures/brick-wall/brick-wall_ao.png"),
                    Texture::SRGBA_UNORM,
                )),
            )],
            floor_transform,
        );

        // LIGHT

        let light = PointLight::new((7.5, 5.0, -4.0), (150.0, 0.0, 0.0));
        let second_light = PointLight::new((-5.0, 4.0, 10.0), (0.0, 0.0, 150.0));
        let third_light = PointLight::new((-1.5, 5.0, 2.0), (150.0, 150.0, 150.0));

        // NEW SKYBOX WITH HDR

        let skybox_loader = SkyboxLoader::new(device);

        let skybox = skybox_loader.load(
            device,
            queue,
            Path::new("./assets/skybox/studio_2k.hdr"),
            512,
        );

        // SCENE

        let entities = vec![helmet, flat_cube, stone_cube, floor, shiba];

        let lights = vec![light, second_light, third_light];

        let scene = Scene {
            entities,
            lights,
            skybox,
        };

        let model_pass = PbrPass::new(device, surface.config(), &layouts);
        let skybox_pass = SkyboxPass::new(device, &layouts);
        let shadow_pass = ShadowPass::new(device, &layouts);

        let hdr_pipeline = HdrPipeline::new(device, surface.config(), &layouts);

        App {
            layouts,
            camera_controller,
            camera,
            scene,
            model_pass,
            skybox_pass,
            shadow_pass,
            hdr_pipeline,
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
            &self.scene.skybox,
        );

        self.generate_shadow_maps(device, queue, &render_world);
        self.skybox_pass.draw(
            device,
            queue,
            self.hdr_pipeline.view(),
            &render_world,
            &render_world.camera,
        );
        self.model_pass.draw(
            device,
            queue,
            self.hdr_pipeline.view(),
            &render_world,
            &render_world.camera,
        );

        self.hdr_pipeline.process(device, queue, view);
    }

    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        self.camera.update_aspect(width as f32 / height as f32);

        self.model_pass.resize(device, width, height);
        self.hdr_pipeline.resize(device, width, height);
    }

    fn generate_shadow_maps(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_world: &RenderWorld,
    ) {
        for light in render_world.lights.iter() {
            let shadow_map = &light.shadow_map;
            for (camera_index, camera) in light.shadow_cameras.iter().enumerate() {
                let shadow_map_view = &shadow_map.create_face_view(camera_index);
                self.shadow_pass
                    .draw(device, queue, shadow_map_view, render_world, camera);
            }
        }
    }
}
