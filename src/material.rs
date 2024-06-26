use std::ops::Deref;

use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BufferUsages, Device,
};

use crate::texture::{Texture, TextureType};

pub struct Material {
    // pub base_color: [f32; 4],
    // pub base_texture: Option<Texture>,
    bind_group: BindGroup,
}

impl Material {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        device: &Device,
        layout: &BindGroupLayout,
        base_color: [f32; 4],
        base_texture: Option<Texture>,
        normal_texture: Option<Texture>,
        metallic_factor: f32,
        roughness_factor: f32,
        metallic_roughness_texture: Option<Texture>,
        ambient_occlussion_texture: Option<Texture>,
    ) -> Material {
        let uniform = MaterialUniform {
            base_color,
            metallic_factor,
            roughness_factor,
            _padding: 0.0,
            _padding2: 0.0,
        };

        let uniform_data = unsafe {
            std::slice::from_raw_parts(
                &uniform as *const MaterialUniform as *const u8,
                std::mem::size_of::<MaterialUniform>(),
            )
        };

        let material_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Material buffer"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            contents: uniform_data,
        });

        let empty_texture = Texture::new(device, 1, 1, None, TextureType::Diffuse);

        let base_texture = match &base_texture {
            Some(texture) => texture,
            None => &empty_texture,
        };

        let normal_texture = match &normal_texture {
            Some(texture) => texture,
            None => &empty_texture,
        };

        let metallic_roughness_texture = match &metallic_roughness_texture {
            Some(texture) => texture,
            None => &empty_texture,
        };

        let ambient_occlussion_texture = match &ambient_occlussion_texture {
            Some(texture) => texture,
            None => &empty_texture,
        };

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Material bind group"),
            layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: material_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&base_texture.sampler),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&base_texture.view),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&normal_texture.sampler),
                },
                BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::TextureView(&normal_texture.view),
                },
                BindGroupEntry {
                    binding: 5,
                    resource: wgpu::BindingResource::Sampler(&metallic_roughness_texture.sampler),
                },
                BindGroupEntry {
                    binding: 6,
                    resource: wgpu::BindingResource::TextureView(&metallic_roughness_texture.view),
                },
                BindGroupEntry {
                    binding: 7,
                    resource: wgpu::BindingResource::Sampler(&ambient_occlussion_texture.sampler),
                },
                BindGroupEntry {
                    binding: 8,
                    resource: wgpu::BindingResource::TextureView(&ambient_occlussion_texture.view),
                },
            ],
        });

        Material {
            // base_color,
            // base_texture,
            bind_group,
        }
    }
}

impl Deref for Material {
    type Target = BindGroup;

    fn deref(&self) -> &Self::Target {
        &self.bind_group
    }
}

#[allow(dead_code)]
#[repr(C)]
struct MaterialUniform {
    base_color: [f32; 4],
    metallic_factor: f32,
    _padding: f32,
    roughness_factor: f32,
    _padding2: f32,
}
