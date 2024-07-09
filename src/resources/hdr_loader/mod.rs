use std::path::Path;

use wgpu::{
    include_wgsl, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, ComputePassDescriptor, ComputePipeline,
    ComputePipelineDescriptor, Device, PipelineLayoutDescriptor, Queue, ShaderStages,
    StorageTextureAccess, TextureFormat, TextureSampleType, TextureUsages, TextureViewDescriptor,
    TextureViewDimension,
};

use crate::texture::CubeMap;

use super::Resources;

pub struct HdrLoader {
    texture_format: TextureFormat,
    bind_group_layout: BindGroupLayout,
    pipeline: ComputePipeline,
}

impl HdrLoader {
    pub fn new(device: &Device) -> Self {
        let module = device.create_shader_module(include_wgsl!("equirectangular.wgsl"));

        let texture_format = CubeMap::RGBA_32_FLOAT;
        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("HdrLoader::equirect_layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: false },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::StorageTexture {
                        access: StorageTextureAccess::WriteOnly,
                        format: texture_format,
                        view_dimension: TextureViewDimension::D2Array,
                    },
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("HDR loader pipeline"),
            layout: Some(&pipeline_layout),
            module: &module,
            entry_point: "compute_equirect_to_cubemap",
            compilation_options: Default::default(),
        });

        Self {
            pipeline,
            texture_format,
            bind_group_layout,
        }
    }

    pub fn load(
        &self,
        device: &Device,
        queue: &Queue,
        path: &Path,
        dst_size: u32,
        label: Option<&str>,
    ) -> CubeMap {
        let src = Resources::load_hdr_texture(device, queue, path);

        let dst = CubeMap::new(
            device,
            dst_size,
            dst_size,
            self.texture_format,
            TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING,
            label,
        );

        let dst_view = dst.texture.create_view(&TextureViewDescriptor {
            label,
            dimension: Some(TextureViewDimension::D2Array),
            ..Default::default()
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label,
            layout: &self.bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&src.view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&dst_view),
                },
            ],
        });

        let mut encoder = device.create_command_encoder(&Default::default());
        let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label,
            ..Default::default()
        });

        let num_workgroups = (dst_size + 15) / 16;
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups(num_workgroups, num_workgroups, 6);

        drop(pass);

        queue.submit([encoder.finish()]);

        dst
    }
}
