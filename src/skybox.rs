use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Device, IndexFormat,
    RenderPass,
};

use crate::{model::Mesh, texture::CubeMap};

pub struct Skybox {
    // _cubemap: CubeMap,
    mesh: Mesh,
    bind_group: BindGroup,
}

impl Skybox {
    pub fn new(device: &Device, layout: &BindGroupLayout, cubemap: &CubeMap) -> Skybox {
        let mesh = Mesh::cube(device);

        let bind_group_entry = BindGroupEntry {
            binding: 1,
            resource: wgpu::BindingResource::TextureView(&cubemap.view),
        };

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Skybox bind group"),
            layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                bind_group_entry,
            ],
        });

        Skybox {
            // _cubemap: cubemap,
            mesh,
            bind_group,
        }
    }
}

pub trait DrawSkybox<'a> {
    fn draw_skybox(&mut self, skybox: &'a Skybox);
}

impl<'a> DrawSkybox<'a> for RenderPass<'a> {
    fn draw_skybox(&mut self, skybox: &'a Skybox) {
        self.set_vertex_buffer(0, skybox.mesh.vertex_buffer.slice(..));
        self.set_index_buffer(skybox.mesh.index_buffer.slice(..), IndexFormat::Uint16);
        self.set_bind_group(1, &skybox.bind_group, &[]);
        self.draw_indexed(0..skybox.mesh.indices_len, 0, 0..1);
    }
}
