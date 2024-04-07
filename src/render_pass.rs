use wgpu::{
    BindGroupLayout, DepthBiasState, DepthStencilState, Device, FragmentState, MultisampleState,
    Operations, PipelineLayoutDescriptor, PrimitiveState, Queue, RenderPassDepthStencilAttachment,
    RenderPipeline, StencilState, Surface, SurfaceConfiguration, VertexState,
};

use crate::{
    camera::Camera,
    model::{DrawModel, Model, Vertex},
    texture::Texture,
    transform::Transform,
};

pub struct RenderPass<'a> {
    device: &'a Device,
    queue: &'a Queue,
    surface: &'a Surface<'a>,
    config: &'a mut SurfaceConfiguration,
    render_pipeline: RenderPipeline,
    depth_texture: Texture,
}

impl<'a> RenderPass<'a> {
    pub fn new(
        device: &'a Device,
        queue: &'a Queue,
        surface: &'a Surface,
        config: &'a mut SurfaceConfiguration,
        bind_group_layouts: &[&BindGroupLayout],
    ) -> RenderPass<'a> {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // DEPTH TEXTURE
        let depth_texture =
            Texture::new_depth_texture(device, config.width, config.height, Some("Depth texture"));

        // PIPELINE
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Pipeline layout"),
            bind_group_layouts,
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
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

        RenderPass {
            device,
            queue,
            surface,
            config,
            render_pipeline,
            depth_texture,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
        }

        self.depth_texture = Texture::new_depth_texture(
            self.device,
            self.config.width,
            self.config.height,
            Some("Depth texture"),
        );

        self.surface.configure(self.device, self.config);
    }

    pub fn draw(&self, models: &[(&Model, &Transform)], camera: &Camera) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
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
        render_pass.set_bind_group(0, &camera.view_projection_bind_group, &[]);

        for (model, transform) in models {
            render_pass.set_bind_group(1, transform, &[]);
            render_pass.draw_model(model);
        }

        drop(render_pass);
        let encoder = encoder.finish();

        self.queue.submit(std::iter::once(encoder));
        output.present();
    }
}
