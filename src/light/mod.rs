mod shadow_camera;

use cgmath::Vector3;
use wgpu::Device;

use crate::texture::CubeMap;

use shadow_camera::ShadowCamera;

pub struct PointLight {
    position: (f32, f32, f32),
    color: (f32, f32, f32),
    // TODO: move in extracted RenderWorld
    pub shadow_map: CubeMap,
    pub shadow_cameras: [ShadowCamera; 6],
}

impl PointLight {
    pub fn new(device: &Device, position: (f32, f32, f32), color: (f32, f32, f32)) -> PointLight {
        let width = 1024;
        let height = 1024;
        let label = Some("Point light shadow map");

        let shadow_map = CubeMap::new_depth_cubemap(device, width, height, label);

        // (look direction, up direction)
        let shadow_cameras_directions = [
            (Vector3::<f32>::unit_x(), Vector3::<f32>::unit_y()),
            (-Vector3::<f32>::unit_x(), Vector3::<f32>::unit_y()),
            (Vector3::<f32>::unit_y(), Vector3::<f32>::unit_z()),
            (-Vector3::<f32>::unit_y(), Vector3::<f32>::unit_z()),
            (Vector3::<f32>::unit_z(), Vector3::<f32>::unit_y()),
            (-Vector3::<f32>::unit_z(), Vector3::<f32>::unit_y()),
        ];

        let shadow_cameras = shadow_cameras_directions
            .map(|(look_dir, up)| ShadowCamera::new(device, position, look_dir, up));

        PointLight {
            position,
            color,
            shadow_map,
            shadow_cameras,
        }
    }

    pub fn to_raw(&self) -> PointLightRaw {
        PointLightRaw {
            position: self.position,
            _padding1: 0.0,
            color: self.color,
            _padding2: 0.0,
        }
    }
}

#[repr(C)]
pub struct PointLightRaw {
    position: (f32, f32, f32),
    _padding1: f32,
    color: (f32, f32, f32),
    _padding2: f32,
}
