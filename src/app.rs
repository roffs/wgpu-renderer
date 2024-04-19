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
    gpu_context::GpuContext,
    layouts::{Layout, Layouts},
    light::PointLight,
    material::Material,
    model::{Mesh, Model},
    render_pass::{self, RenderPasses},
    resources::Resources,
    scene::Scene,
    skybox::Skybox,
    surface_context::SurfaceContext,
    transform::{Rotation, Transform},
};

pub struct App<'a> {
    _layouts: Layouts,
    camera_controller: CameraController,
    camera: Camera,
    scene: Scene,
    render_passes: RenderPasses<'a>,
}

impl<'a> App<'a> {
    pub fn new(context: &'a GpuContext, surface: &SurfaceContext) -> App<'a> {
        let layouts = Layouts::new(&context.device);
        let render_passes =
            RenderPasses::new(&context.device, &context.queue, surface.config(), &layouts);

        // CAMERA
        let camera_controller = CameraController::new(0.1, 0.1);
        let camera = Camera::new(
            (0.0, 1.0, 3.0),
            Deg(-90.0),
            Deg(0.0),
            45.0,
            surface.config().width as f32 / surface.config().height as f32,
            0.01,
            100.0,
        );

        // MODELS
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

        // LIGHT

        let light = PointLight::new((1.0, 5.0, 0.0), (1.0, 1.0, 1.0));
        let second_light = PointLight::new((-1.0, 1.0, 1.0), (1.0, 1.0, 1.0));

        // SKYBOX

        let skybox_paths = [
            Path::new("./assets/skybox/sky/right.jpg"),
            Path::new("./assets/skybox/sky/left.jpg"),
            Path::new("./assets/skybox/sky/top.jpg"),
            Path::new("./assets/skybox/sky/bottom.jpg"),
            Path::new("./assets/skybox/sky/front.jpg"),
            Path::new("./assets/skybox/sky/back.jpg"),
        ];

        let skybox_cubemap =
            Resources::load_cube_map(&context.device, &context.queue, skybox_paths);
        let skybox = Skybox::new(
            &context.device,
            layouts.get(&Layout::Skybox),
            skybox_cubemap,
        );

        // SCENE

        let entities = vec![
            (shiba, transform_matrix),
            (flat_cube, transform_matrix_2),
            (stone_cube, transform_matrix_3),
            (floor, floor_transform),
        ];

        let lights = vec![light, second_light];

        let scene = Scene {
            entities,
            lights,
            skybox,
        };
        App {
            _layouts: layouts,
            render_passes,
            camera_controller,
            camera,
            scene,
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

    pub fn render(&mut self, view: &TextureView) {
        self.camera_controller.update(&mut self.camera);

        let model_pass = self.render_passes.get(&render_pass::PassKind::Model);
        let skybox_pass = self.render_passes.get(&render_pass::PassKind::Skybox);

        skybox_pass.draw(view, &self.camera, &self.scene);
        model_pass.draw(view, &self.camera, &self.scene);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.camera.update_aspect(width as f32 / height as f32);
        self.render_passes.resize(width, height)
    }
}
