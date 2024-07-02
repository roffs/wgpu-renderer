use std::ops::Deref;

use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Device};

use crate::environment_map::EnvironmentMap;

pub struct ExtractedEnvMap {
    bind_group: BindGroup,
}

impl ExtractedEnvMap {
    pub fn new(
        device: &Device,
        layout: &BindGroupLayout,
        env_map: &EnvironmentMap,
    ) -> ExtractedEnvMap {
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Skybox bind group"),
            layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Sampler(&env_map.sampler),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&env_map.view),
                },
            ],
        });

        ExtractedEnvMap { bind_group }
    }
}

impl Deref for ExtractedEnvMap {
    type Target = BindGroup;

    fn deref(&self) -> &Self::Target {
        &self.bind_group
    }
}
