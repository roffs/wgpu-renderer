use wgpu::{
    include_wgsl, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, CommandEncoderDescriptor,
    ComputePassDescriptor, ComputePipeline, ComputePipelineDescriptor, Device,
    PipelineLayoutDescriptor, PushConstantRange, Queue, ShaderStages, StorageTextureAccess,
    TextureFormat, TextureSampleType, TextureUsages, TextureViewDescriptor, TextureViewDimension,
};

use crate::texture::CubeMap;

pub struct IrrMapGenerator {
    texture_format: TextureFormat,
    bind_group_layout: BindGroupLayout,
    pipeline: ComputePipeline,
}

impl IrrMapGenerator {
    pub fn new(device: &Device) -> IrrMapGenerator {
        let shader = device.create_shader_module(include_wgsl!("convolution.wgsl"));

        let texture_format = CubeMap::RGBA_32_FLOAT;

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("HdrLoader::equirect_layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: false },
                        view_dimension: TextureViewDimension::Cube,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
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
            label: Some("Irradiation map pipeline layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[PushConstantRange {
                stages: ShaderStages::COMPUTE,
                range: 0..4,
            }],
        });

        let pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("Irradiation map pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "compute_irr_map",
            compilation_options: Default::default(),
            cache: None,
        });

        Self {
            pipeline,
            texture_format,
            bind_group_layout,
        }
    }

    pub fn generate(
        &self,
        device: &Device,
        queue: &Queue,
        env_map: &CubeMap,
        dst_size: u32,
    ) -> CubeMap {
        let irr_map = CubeMap::new(
            device,
            dst_size,
            dst_size,
            self.texture_format,
            TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING,
            Some("Irradiance map"),
        );

        let env_map_view = env_map.texture.create_view(&TextureViewDescriptor {
            label: Some("Environment cube map view"),
            dimension: Some(TextureViewDimension::Cube),
            ..Default::default()
        });

        let irr_map_view = irr_map.texture.create_view(&TextureViewDescriptor {
            label: Some("Irradiance map view"),
            dimension: Some(TextureViewDimension::D2Array),
            ..Default::default()
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Environment map bind group"),
            layout: &self.bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&env_map_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&env_map.sampler),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::TextureView(&irr_map_view),
                },
            ],
        });

        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Irradiance map Encoder"),
        });
        let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("Irradiance map compute pass"),
            ..Default::default()
        });

        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &bind_group, &[]);

        for face_index in 0..6_u32 {
            pass.set_push_constants(0, &face_index.to_ne_bytes());
            pass.dispatch_workgroups(dst_size / 16, dst_size / 16, 1);
        }

        drop(pass);

        let encoder = encoder.finish();

        queue.submit(std::iter::once(encoder));

        irr_map
    }
}
