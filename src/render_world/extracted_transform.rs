use std::ops::Deref;

use cgmath::{Matrix, Matrix4, SquareMatrix};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BufferUsages, Device,
};

use crate::transform::Transform;

pub struct ExtractedTransform {
    bind_group: BindGroup,
}

impl ExtractedTransform {
    pub fn new(
        device: &Device,
        layout: &BindGroupLayout,
        transform: &Transform,
        parent_model_matrix: Matrix4<f32>,
    ) -> ExtractedTransform {
        let mut local_model = transform.model();
        let mut normal_matrix = local_model.invert().unwrap().transpose();

        let parent_normal_matrix = parent_model_matrix.invert().unwrap().transpose();

        local_model = parent_model_matrix * local_model;
        normal_matrix = normal_matrix * parent_normal_matrix;

        let uniform = TransformUniform {
            model_matrix: local_model,
            normal_matrix,
        };

        let buffer_data = unsafe {
            std::slice::from_raw_parts(
                &uniform as *const TransformUniform as *const u8,
                std::mem::size_of::<TransformUniform>(),
            )
        };
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Model buffer"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            contents: buffer_data,
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Model bind group"),
            layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        ExtractedTransform { bind_group }
    }
}

impl Deref for ExtractedTransform {
    type Target = BindGroup;

    fn deref(&self) -> &Self::Target {
        &self.bind_group
    }
}

#[allow(dead_code)]
struct TransformUniform {
    model_matrix: cgmath::Matrix4<f32>,
    normal_matrix: cgmath::Matrix4<f32>,
}
