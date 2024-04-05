use cgmath::{Deg, InnerSpace, Vector3, Zero};

use super::Camera;

pub struct CameraController {
    move_speed: f32,
    rotation_speed: f32,

    pub move_direction: Vector3<f32>,
    pub pitch_delta: f32,
    pub yaw_delta: f32,
}

impl CameraController {
    pub fn new(move_speed: f32, rotation_speed: f32) -> CameraController {
        CameraController {
            move_speed,
            rotation_speed,
            move_direction: Vector3::zero(),
            pitch_delta: 0.0,
            yaw_delta: 0.0,
        }
    }

    pub fn update(&mut self, camera: &mut Camera) {
        self.translate(camera, self.move_direction);
        self.rotate(camera, self.yaw_delta, self.pitch_delta);
        camera.update_buffer();
    }

    fn translate(&self, camera: &mut Camera, local_direction: cgmath::Vector3<f32>) {
        let mut direction = (camera.forward * local_direction.x)
            + (camera.up * local_direction.y)
            + (camera.right * local_direction.z);

        if direction != Vector3::zero() {
            direction = direction.normalize();
        }
        camera.position += direction * self.move_speed;
    }

    fn rotate(&mut self, camera: &mut Camera, yaw: f32, pitch: f32) {
        camera.yaw += Deg(yaw * self.rotation_speed).into();

        camera.pitch -= Deg(pitch * self.rotation_speed).into();
        camera.pitch = match camera.pitch {
            angle if angle < Deg(-89.0).into() => Deg(-89.0).into(),
            angle if angle > Deg(89.0).into() => Deg(89.0).into(),
            angle => angle,
        };

        camera.update_directions();

        self.pitch_delta = 0.0;
        self.yaw_delta = 0.0;
    }
}
