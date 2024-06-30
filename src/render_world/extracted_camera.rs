use std::ops::Deref;

use cgmath::Matrix4;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Buffer, BufferUsages, Device,
};

use crate::camera::Camera;

#[allow(dead_code)]
pub struct ExtractedCamera {
    buffer: Buffer,
    bind_group: BindGroup,
}

impl ExtractedCamera {
    pub fn new(device: &Device, layout: &BindGroupLayout, camera: &Camera) -> ExtractedCamera {
        let camera_uniform = CameraUniform::from(camera);
        let camera_uniform = unsafe {
            std::slice::from_raw_parts(
                &camera_uniform as *const CameraUniform as *const u8,
                std::mem::size_of::<CameraUniform>(),
            )
        };

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Model camera buffer"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            contents: camera_uniform,
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Model camera bind group"),
            layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        ExtractedCamera { buffer, bind_group }
    }
}

impl Deref for ExtractedCamera {
    type Target = BindGroup;

    fn deref(&self) -> &Self::Target {
        &self.bind_group
    }
}

#[allow(dead_code)]
#[repr(C)]
pub struct CameraUniform {
    view: Matrix4<f32>,
    rotation: Matrix4<f32>,
    projection: Matrix4<f32>,
    position: [f32; 3],
    _padding: f32,
}

impl From<&Camera> for CameraUniform {
    fn from(camera: &Camera) -> Self {
        CameraUniform {
            view: camera.get_view(),
            rotation: camera.get_rotation(),
            projection: camera.get_projection(),
            position: camera.position.into(),
            _padding: 0.0,
        }
    }
}
