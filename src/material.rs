use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Device};

use crate::texture::Texture;

pub struct Material {
    pub diffuse: Texture,

    pub bind_group: BindGroup,
}

impl Material {
    pub fn new(device: &Device, layout: &BindGroupLayout, diffuse: Texture) -> Material {
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Texture bind group"),
            layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Sampler(&diffuse.sampler),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&diffuse.view),
                },
            ],
        });

        Material {
            diffuse,
            bind_group,
        }
    }
}
