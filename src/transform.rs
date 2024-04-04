use std::ops::Deref;

use cgmath::{Deg, Matrix, Matrix4};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BufferDescriptor,
    BufferUsages, Device, Queue,
};

pub struct Transform {
    bind_group: BindGroup,
}

impl Transform {
    pub fn new(
        device: &Device,
        queue: &Queue,
        layout: &BindGroupLayout,
        translation: (f32, f32, f32),
        scale: f32,
    ) -> Transform {
        let translation_matrix =
            cgmath::Matrix4::from_translation(cgmath::Vector3::<f32>::from(translation));
        let rotation_matrix = cgmath::Matrix4::<f32>::from_angle_z(Deg(15.0)); // TODO put rotation in constructor
        let scale_matrix = cgmath::Matrix4::<f32>::from_scale(scale);

        let transform_matrix = translation_matrix * rotation_matrix * scale_matrix;

        let buffer_size = std::mem::size_of::<cgmath::Matrix4<f32>>();
        let buffer_data = unsafe {
            std::slice::from_raw_parts(transform_matrix.as_ptr() as *const u8, buffer_size)
        };

        let model_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Model buffer"),
            size: std::mem::size_of::<Matrix4<f32>>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Model bind group"),
            layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: model_buffer.as_entire_binding(),
            }],
        });

        queue.write_buffer(&model_buffer, 0, buffer_data);

        Transform { bind_group }
    }
}

impl Deref for Transform {
    type Target = BindGroup;

    fn deref(&self) -> &Self::Target {
        &self.bind_group
    }
}
