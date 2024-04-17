use std::ops::Deref;

use cgmath::{Deg, Matrix, SquareMatrix};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BufferDescriptor,
    BufferUsages, Device, Queue,
};

#[allow(dead_code)]
pub enum Rotation {
    X(f32),
    Y(f32),
    Z(f32),
}

pub struct Transform {
    bind_group: BindGroup,
}

impl Transform {
    pub fn new(
        device: &Device,
        queue: &Queue,
        layout: &BindGroupLayout,
        translation: (f32, f32, f32),
        rotation: Option<Rotation>,
        scale: f32,
    ) -> Transform {
        let translation_matrix =
            cgmath::Matrix4::from_translation(cgmath::Vector3::<f32>::from(translation));
        let scale_matrix = cgmath::Matrix4::<f32>::from_scale(scale);

        let rotation_matrix = if let Some(rotation) = rotation {
            match rotation {
                Rotation::X(deg) => cgmath::Matrix4::<f32>::from_angle_x(Deg(deg)),
                Rotation::Y(deg) => cgmath::Matrix4::<f32>::from_angle_y(Deg(deg)),
                Rotation::Z(deg) => cgmath::Matrix4::<f32>::from_angle_z(Deg(deg)),
            }
        } else {
            cgmath::Matrix4::identity()
        };

        let model_matrix = translation_matrix * rotation_matrix * scale_matrix;
        let normal_matrix = model_matrix.invert().unwrap().transpose();

        let uniform = TransformUniform {
            _model_matrix: model_matrix,
            _normal_matrix: normal_matrix,
        };

        let buffer_size = std::mem::size_of::<TransformUniform>();
        let buffer_data = unsafe {
            std::slice::from_raw_parts(
                &uniform as *const TransformUniform as *const u8,
                buffer_size,
            )
        };

        let buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Model buffer"),
            size: std::mem::size_of::<TransformUniform>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Model bind group"),
            layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        queue.write_buffer(&buffer, 0, buffer_data);

        Transform { bind_group }
    }
}

impl Deref for Transform {
    type Target = BindGroup;

    fn deref(&self) -> &Self::Target {
        &self.bind_group
    }
}

struct TransformUniform {
    _model_matrix: cgmath::Matrix4<f32>,
    _normal_matrix: cgmath::Matrix4<f32>,
}
