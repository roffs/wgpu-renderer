use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, Color, DepthBiasState, DepthStencilState,
    Device, FragmentState, MultisampleState, Operations, PipelineLayoutDescriptor, PrimitiveState,
    RenderPassDepthStencilAttachment, RenderPipeline, StencilState, SurfaceConfiguration,
    VertexState,
};

use crate::{
    entity::{DrawEntity, Vertex},
    layouts::Layouts,
    light::PointLight,
    texture::Texture,
};

use super::RenderPass;

pub struct ShadowPass {
    pipeline: RenderPipeline,
    depth_texture: Texture,
    shadow_bind_groups: Vec<BindGroup>,
}

impl ShadowPass {
    pub fn new(
        device: &Device,
        config: &SurfaceConfiguration,
        layouts: &Layouts,
        lights: &Vec<PointLight>,
    ) -> ShadowPass {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shadow.wgsl").into()),
        });

        let mut shadow_bind_groups = Vec::new();

        for light in lights {
            for shadow_camera in &light.shadow_cameras {
                let shadow_bind_group = device.create_bind_group(&BindGroupDescriptor {
                    label: Some("Shadow bind group"),
                    layout: &layouts.shadow_cube_map,
                    entries: &[
                        BindGroupEntry {
                            binding: 0,
                            resource: shadow_camera.view_buffer.as_entire_binding(),
                        },
                        BindGroupEntry {
                            binding: 1,
                            resource: shadow_camera.proj_buffer.as_entire_binding(),
                        },
                    ],
                });

                shadow_bind_groups.push(shadow_bind_group);
            }
        }

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Pipeline layout"),
            bind_group_layouts: &[&layouts.shadow_cube_map, &layouts.transform],
            push_constant_ranges: &[],
        });

        // DEPTH TEXTURE
        let depth_texture = Texture::new_depth_texture(device, 1024, 1024, Some("Depth texture"));

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                front_face: wgpu::FrontFace::Cw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: true,
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
            multisample: MultisampleState::default(),
            multiview: None,
        });

        ShadowPass {
            pipeline,
            depth_texture,
            shadow_bind_groups,
        }
    }
}

impl RenderPass for ShadowPass {
    fn draw(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _view: &wgpu::TextureView,
        _camera: &crate::camera::Camera,
        scene: &crate::scene::Scene,
    ) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Shadow map render Encoder"),
        });

        for (light_index, light) in scene.lights.iter().enumerate() {
            for camera_index in 0..light.shadow_cameras.len() {
                let shadow_map_view =
                    light
                        .shadow_map
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor {
                            label: Some(
                                format!("Light {} shadow view {}", light_index, camera_index)
                                    .as_str(),
                            ),
                            dimension: Some(wgpu::TextureViewDimension::D2),
                            base_array_layer: camera_index as u32,
                            array_layer_count: Some(1),
                            ..Default::default()
                        });

                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Shadow map render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &shadow_map_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(Color::WHITE),
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

                render_pass.set_pipeline(&self.pipeline);

                let shadow_bind_group = &self
                    .shadow_bind_groups
                    .get((light_index * 6) + camera_index)
                    .unwrap();
                render_pass.set_bind_group(0, shadow_bind_group, &[]);

                for (entity, transform) in &scene.entities {
                    render_pass.set_bind_group(1, transform, &[]);
                    render_pass.draw_entity(entity);
                }
            }
        }

        let encoder = encoder.finish();

        queue.submit(std::iter::once(encoder));
    }

    fn resize(&mut self, _device: &wgpu::Device, _width: u32, _height: u32) {}
}
