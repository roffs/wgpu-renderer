use cgmath::Vector3;
use wgpu::Device;

use crate::{camera::Camera, layouts::Layouts, light::PointLight, texture::CubeMap};

use super::ExtractedCamera;

pub struct ExtractedPointLight {
    pub uniform: PointLightUniform,
    pub shadow_map: CubeMap,
    pub shadow_cameras: [ExtractedCamera; 6],
}

impl ExtractedPointLight {
    pub fn new(
        device: &Device,
        layouts: &Layouts,
        point_light: &PointLight,
    ) -> ExtractedPointLight {
        let uniform = PointLightUniform::from(point_light);

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

        let shadow_cameras = shadow_cameras_directions.map(|(look_dir, up)| {
            let camera = Camera::new_from_look_direction(
                point_light.position,
                look_dir,
                up,
                90.0,
                1.0,
                0.5,
                25.0,
            );
            ExtractedCamera::new(device, &layouts.camera, &camera)
        });

        ExtractedPointLight {
            // buffer,
            uniform,
            shadow_map,
            shadow_cameras,
        }
    }
}

#[repr(C)]
pub struct PointLightUniform {
    position: (f32, f32, f32),
    _padding1: f32,
    color: (f32, f32, f32),
    _padding2: f32,
}

impl From<&PointLight> for PointLightUniform {
    fn from(point_light: &PointLight) -> Self {
        PointLightUniform {
            position: point_light.position,
            _padding1: 0.0,
            color: point_light.color,
            _padding2: 0.0,
        }
    }
}

// pub struct ShadowCamera {
//     pub view_buffer: Buffer,
//     pub proj_buffer: Buffer,
//     pub bind_group: BindGroup,
// }

// impl ShadowCamera {
//     pub fn new<T: Into<Point3<f32>>, U: Into<Vector3<f32>>>(
//         device: &Device,
//         layout: &BindGroupLayout,
//         position: T,
//         look_dir: U,
//         up: U,
//     ) -> ShadowCamera {
//         let view: Matrix4<f32> = Matrix4::look_to_rh(position.into(), look_dir.into(), up.into());
//         let projection: Matrix4<f32> = perspective(Deg(90.0), 1.0, 0.5, 25.0);
//         let invert_x_axis: Matrix4<f32> = Matrix4::from_nonuniform_scale(-1.0, 1.0, 1.0); // Correcting the reflection direction as we are viewing the skybox from the inside

//         let view_buffer = {
//             let data = view;
//             let data = unsafe {
//                 std::slice::from_raw_parts(
//                     data.as_ptr() as *const u8,
//                     std::mem::size_of::<Matrix4<f32>>(),
//                 )
//             };

//             device.create_buffer_init(&BufferInitDescriptor {
//                 label: Some("Shadow camera view projection buffer"),
//                 contents: data,
//                 usage: BufferUsages::UNIFORM,
//             })
//         };

//         let proj_buffer = {
//             let data = invert_x_axis * projection;
//             let data = unsafe {
//                 std::slice::from_raw_parts(
//                     data.as_ptr() as *const u8,
//                     std::mem::size_of::<Matrix4<f32>>(),
//                 )
//             };

//             device.create_buffer_init(&BufferInitDescriptor {
//                 label: Some("Shadow camera view projection buffer"),
//                 contents: data,
//                 usage: BufferUsages::UNIFORM,
//             })
//         };

//         let bind_group = device.create_bind_group(&BindGroupDescriptor {
//             label: Some("Shadow bind group"),
//             layout,
//             entries: &[
//                 BindGroupEntry {
//                     binding: 0,
//                     resource: view_buffer.as_entire_binding(),
//                 },
//                 BindGroupEntry {
//                     binding: 1,
//                     resource: proj_buffer.as_entire_binding(),
//                 },
//             ],
//         });

//         ShadowCamera {
//             view_buffer,
//             proj_buffer,
//             bind_group,
//         }
//     }
// }
