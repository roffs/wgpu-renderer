use cgmath::Deg;

use super::Camera;

pub struct CameraController {
    move_speed: f32,
    rotation_speed: f32,
}

impl CameraController {
    pub fn new(move_speed: f32, rotation_speed: f32) -> CameraController {
        CameraController {
            move_speed,
            rotation_speed,
        }
    }

    pub fn translate(&self, camera: &mut Camera, local_direction: cgmath::Vector3<f32>) {
        let direction = (camera.forward * local_direction.x)
            + (camera.up * local_direction.y)
            + (camera.right * local_direction.z);
        camera.position += direction * self.move_speed;
    }

    pub fn rotate(&self, camera: &mut Camera, delta: (f32, f32)) {
        let (yaw, pitch) = delta;

        camera.yaw += Deg(yaw * self.rotation_speed).into();

        camera.pitch -= Deg(pitch * self.rotation_speed).into();
        camera.pitch = match camera.pitch {
            angle if angle < Deg(-90.0).into() => Deg(-90.0).into(),
            angle if angle > Deg(90.0).into() => Deg(90.0).into(),
            angle => angle,
        };

        camera.update_directions();
    }
}
