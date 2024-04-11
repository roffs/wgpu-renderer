use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BufferUsages, Color, Device,
    Queue,
};

use crate::texture::Texture;

pub struct Material {
    pub base_color: Color,
    pub base_texture: Option<Texture>,

    pub bind_group: BindGroup,
}

impl Material {
    pub fn new(
        device: &Device,
        queue: &Queue,
        layout: &BindGroupLayout,
        base_color: Color,
        base_texture: Option<Texture>,
    ) -> Material {
        let color_data = unsafe {
            std::slice::from_raw_parts(
                &(
                    base_color.r as f32,
                    base_color.g as f32,
                    base_color.b as f32,
                    base_color.a as f32,
                ) as *const (f32, f32, f32, f32) as *const u8,
                std::mem::size_of::<f32>() * 4,
            )
        };

        let base_color_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Base color buffer"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            contents: color_data,
        });

        let empty_texture = Texture::new(device, queue, 1, 1, &[0, 0, 0, 0], None);

        let texture = match &base_texture {
            Some(texture) => texture,
            None => &empty_texture,
        };

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Material bind group"),
            layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(
                        base_color_buffer.as_entire_buffer_binding(),
                    ),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
            ],
        });

        Material {
            base_color,
            base_texture,
            bind_group,
        }
    }
}
