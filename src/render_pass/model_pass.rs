use cgmath::Matrix;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, Buffer, BufferDescriptor, BufferUsages,
    DepthBiasState, DepthStencilState, Device, FragmentState, MultisampleState, Operations,
    PipelineLayoutDescriptor, PrimitiveState, RenderPassDepthStencilAttachment, RenderPipeline,
    Sampler, StencilState, SurfaceConfiguration, TextureView, VertexState,
};

use crate::{
    camera::Camera,
    entity::{DrawEntity, Vertex},
    layouts::Layouts,
    light::{PointLight, PointLightRaw},
    scene::Scene,
    texture::{Texture, TextureType},
};

use super::RenderPass;

pub struct ModelPass {
    render_pipeline: RenderPipeline,
    depth_texture: Texture,
    camera_buffer: Buffer,
    camera_bind_group: BindGroup,
    light_buffer: Buffer,
    light_bind_group: BindGroup,
}

impl ModelPass {
    pub fn new(
        device: &Device,
        config: &SurfaceConfiguration,
        layouts: &Layouts,
        lights: &[PointLight],
    ) -> ModelPass {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/model.wgsl").into()),
        });

        // CAMERA

        let camera_buffer_size = std::mem::size_of::<cgmath::Matrix4<f32>>();

        let camera_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Model camera buffer"),
            size: camera_buffer_size as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let camera_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Model camera bind group"),
            layout: &layouts.transform,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        // LIGHT
        let light_buffer_size = lights.len() * std::mem::size_of::<PointLightRaw>();

        let light_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Model light buffer"),
            size: light_buffer_size as u64,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let view_array = lights
            .iter()
            .map(|light| &light.shadow_map.view)
            .collect::<Vec<&TextureView>>();

        let sampler_array = lights
            .iter()
            .map(|light| &light.shadow_map.sampler)
            .collect::<Vec<&Sampler>>();

        let light_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Model light bind group"),
            layout: &layouts.light,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: light_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureViewArray(&view_array),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::SamplerArray(&sampler_array),
                },
            ],
        });

        // DEPTH TEXTURE
        let depth_texture = Texture::new(
            device,
            config.width,
            config.height,
            Some("Depth texture"),
            TextureType::Depth,
        );

        // PIPELINE
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Pipeline layout"),
            bind_group_layouts: &[
                &layouts.transform,
                &layouts.transform,
                &layouts.material,
                &layouts.light,
            ],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: &[Vertex::desc()],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        ModelPass {
            render_pipeline,
            depth_texture,
            camera_buffer,
            camera_bind_group,
            light_buffer,
            light_bind_group,
        }
    }
}

impl RenderPass for ModelPass {
    fn draw(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        view: &TextureView,
        camera: &Camera,
        scene: &Scene,
    ) {
        let view_projection = camera.get_projection() * camera.get_view();
        let view_projection = unsafe {
            std::slice::from_raw_parts(
                view_projection.as_ptr() as *const u8,
                std::mem::size_of::<cgmath::Matrix4<f32>>(),
            )
        };

        queue.write_buffer(&self.camera_buffer, 0, view_projection);

        // UPDATE LIGHT BUFFER

        let light_size = std::mem::size_of::<PointLightRaw>();

        for (index, light) in scene.lights.iter().enumerate() {
            let light_data = unsafe {
                let light = &light.to_raw();
                std::slice::from_raw_parts(light as *const PointLightRaw as *const u8, light_size)
            };

            queue.write_buffer(&self.light_buffer, (light_size * index) as u64, light_data);
        }

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: &self.depth_texture.view,
                depth_ops: Some(Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
        render_pass.set_bind_group(3, &self.light_bind_group, &[]);

        for (entity, transform) in &scene.entities {
            render_pass.set_bind_group(1, transform, &[]);
            render_pass.draw_entity(entity);
        }

        drop(render_pass);
        let encoder = encoder.finish();

        queue.submit(std::iter::once(encoder));
    }

    fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        self.depth_texture = Texture::new(
            device,
            width,
            height,
            Some("Depth texture"),
            TextureType::Depth,
        );
    }
}
