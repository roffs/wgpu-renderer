use cgmath::{perspective, Deg, Matrix, Matrix4, Point3, Vector3};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, Device,
};

pub struct ShadowCamera {
    pub view_proj_buffer: Buffer,
}

impl ShadowCamera {
    pub fn new<T: Into<Point3<f32>>, U: Into<Vector3<f32>>>(
        device: &Device,
        position: T,
        look_dir: U,
        up: U,
    ) -> ShadowCamera {
        let view = Matrix4::look_to_rh(position.into(), look_dir.into(), up.into());
        let projection = perspective(Deg(90.0), 1.0, 0.5, 25.0);
        let invert_x_axis = Matrix4::from_nonuniform_scale(-1.0, 1.0, 1.0); // Correcting the reflection direction as we are viewing the skybox from the inside

        let data = invert_x_axis * projection * view;
        let data = unsafe {
            std::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                std::mem::size_of::<Matrix4<f32>>(),
            )
        };

        let view_proj_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Shadow camera view projection buffer"),
            contents: data,
            usage: BufferUsages::UNIFORM,
        });

        ShadowCamera { view_proj_buffer }
    }
}
