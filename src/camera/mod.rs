mod camera_controller;
use cgmath::{perspective, Angle, Deg, Matrix4, Point3, Rad, Vector3};

pub use camera_controller::CameraController;

pub struct Camera {
    pub(self) position: Point3<f32>,
    pub(self) yaw: Rad<f32>,
    pub(self) pitch: Rad<f32>,

    pub(self) look_dir: Vector3<f32>,
    pub(self) right: Vector3<f32>,
    pub(self) up: Vector3<f32>,
    pub(self) forward: Vector3<f32>,

    fovy: f32,
    aspect: f32,
    near: f32,
    far: f32,
}

impl Camera {
    pub fn new<T: Into<cgmath::Rad<f32>>>(
        position: (f32, f32, f32),

        yaw: T,
        pitch: T,

        fovy: f32,
        aspect: f32,
        near: f32,
        far: f32,
    ) -> Camera {
        let yaw: Rad<f32> = yaw.into();
        let pitch: Rad<f32> = pitch.into();

        let (look_dir, up, right, forward) = calculate_local_directions(yaw, pitch);

        Camera {
            position: Point3::from(position),
            yaw,
            pitch,
            look_dir,
            right,
            up,
            forward,
            fovy,
            aspect,
            near,
            far,
        }
    }

    pub fn get_position(&self) -> cgmath::Point3<f32> {
        self.position
    }

    pub fn get_rotation(&self) -> cgmath::Matrix4<f32> {
        Matrix4::look_to_rh((0.0, 0.0, 0.0).into(), self.look_dir, self.up)
    }
    pub fn get_view(&self) -> cgmath::Matrix4<f32> {
        Matrix4::look_to_rh(self.position, self.look_dir, self.up)
    }

    pub fn get_projection(&self) -> cgmath::Matrix4<f32> {
        perspective(Deg(self.fovy), self.aspect, self.near, self.far)
    }

    pub(self) fn update_directions(&mut self) {
        let (look_dir, up, right, forward) = calculate_local_directions(self.yaw, self.pitch);

        self.look_dir = look_dir;
        self.up = up;
        self.right = right;
        self.forward = forward;
    }
}

fn calculate_local_directions(
    yaw: Rad<f32>,
    pitch: Rad<f32>,
) -> (Vector3<f32>, Vector3<f32>, Vector3<f32>, Vector3<f32>) {
    let x = yaw.cos() * pitch.cos();
    let y = pitch.sin();
    let z = yaw.sin() * pitch.cos();

    let look_dir = Vector3::new(x, y, z);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let right = look_dir.cross(up);
    let forward = up.cross(right);

    (look_dir, up, right, forward)
}
