use wgpu::{
    include_wgsl, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, ComputePassDescriptor, ComputePipeline,
    ComputePipelineDescriptor, Device, PipelineLayoutDescriptor, Queue, ShaderStages,
    StorageTextureAccess, TextureFormat, TextureSampleType, TextureUsages, TextureViewDescriptor,
    TextureViewDimension,
};

use crate::texture::{CubeMap, Texture};

pub struct HdrLoader {
    texture_format: TextureFormat,
    bind_group_layout: BindGroupLayout,
    pipeline: ComputePipeline,
}

impl HdrLoader {
    pub fn new(device: &Device) -> HdrLoader {
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
            cache: None,
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
        hdr_texture: &Texture,
        dst_size: u32,
    ) -> CubeMap {
        let env_map = CubeMap::new(
            device,
            dst_size,
            dst_size,
            self.texture_format,
            TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING,
            Some("Environment map"),
        );

        let env_map_view = env_map.texture.create_view(&TextureViewDescriptor {
            label: Some("Environment map view"),
            dimension: Some(TextureViewDimension::D2Array),
            ..Default::default()
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Environment map bind group"),
            layout: &self.bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&hdr_texture.view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&env_map_view),
                },
            ],
        });

        let mut encoder = device.create_command_encoder(&Default::default());
        let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("Environment map compute pass"),
            ..Default::default()
        });

        let num_workgroups = (dst_size + 15) / 16;
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups(num_workgroups, num_workgroups, 6);

        drop(pass);

        queue.submit([encoder.finish()]);

        env_map
    }
}
