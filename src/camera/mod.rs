mod camera_controller;
use cgmath::{perspective, Angle, Deg, InnerSpace, Matrix4, Point3, Rad, Vector3};

pub use camera_controller::CameraController;

pub struct Camera {
    pub position: Point3<f32>,
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
    pub fn new<T: Into<Rad<f32>>, U: Into<Point3<f32>>>(
        position: U,
        yaw: T,
        pitch: T,

        fovy: f32,
        aspect: f32,
        near: f32,
        far: f32,
    ) -> Camera {
        let yaw = yaw.into();
        let pitch = pitch.into();

        let up = Vector3::new(0.0, 1.0, 0.0);
        let (look_dir, right, forward) = calculate_local_directions(yaw, pitch, up);

        Camera {
            position: position.into(),
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

    pub fn new_from_look_direction<U: Into<Point3<f32>>>(
        position: U,
        look_direction: Vector3<f32>,
        up: Vector3<f32>,
        fovy: f32,
        aspect: f32,
        near: f32,
        far: f32,
    ) -> Camera {
        let look_direction = look_direction.normalize();

        let pitch = Rad(look_direction.y.asin());
        let yaw = Rad(look_direction.z.atan2(look_direction.x));
        let (look_dir, right, forward) = calculate_local_directions(yaw, pitch, up);

        Camera {
            position: position.into(),
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

    pub fn get_view(&self) -> cgmath::Matrix4<f32> {
        Matrix4::look_to_rh(self.position, self.look_dir, self.up)
    }

    pub fn get_projection(&self) -> cgmath::Matrix4<f32> {
        perspective(Deg(self.fovy), self.aspect, self.near, self.far)
    }

    pub fn update_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    pub(self) fn update_directions(&mut self) {
        let (look_dir, right, forward) = calculate_local_directions(self.yaw, self.pitch, self.up);

        self.look_dir = look_dir;
        self.right = right;
        self.forward = forward;
    }
}

fn calculate_local_directions(
    yaw: Rad<f32>,
    pitch: Rad<f32>,
    up: Vector3<f32>,
) -> (Vector3<f32>, Vector3<f32>, Vector3<f32>) {
    let x = yaw.cos() * pitch.cos();
    let y = pitch.sin();
    let z = yaw.sin() * pitch.cos();

    let look_dir = Vector3::new(x, y, z);
    let right = look_dir.cross(up);
    let forward = up.cross(right);

    (look_dir, right, forward)
}
