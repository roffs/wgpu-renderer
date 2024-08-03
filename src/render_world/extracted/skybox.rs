use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Device};

use crate::skybox::Skybox;

pub struct ExtractedSkybox {
    pub env_map_bind_group: BindGroup,
    pub irr_map_bind_group: BindGroup,
}

impl ExtractedSkybox {
    pub fn new(device: &Device, layout: &BindGroupLayout, skybox: &Skybox) -> ExtractedSkybox {
        let env_map_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Skybox env map bind group"),
            layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Sampler(&skybox.env_map.sampler),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&skybox.env_map.view),
                },
            ],
        });

        let irr_map_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Skybox irr map bind group"),
            layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Sampler(&skybox.irr_map.sampler),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&skybox.irr_map.view),
                },
            ],
        });

        ExtractedSkybox {
            env_map_bind_group,
            irr_map_bind_group,
        }
    }
}
