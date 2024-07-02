use wgpu::{
    BlendState, ColorTargetState, ColorWrites, CommandEncoderDescriptor, CompareFunction,
    DepthBiasState, DepthStencilState, Device, Face, FragmentState, FrontFace, LoadOp,
    MultisampleState, Operations, PipelineLayoutDescriptor, PolygonMode, PrimitiveState,
    PrimitiveTopology, Queue, RenderPassColorAttachment, RenderPassDepthStencilAttachment,
    RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor,
    ShaderSource, StencilState, StoreOp, SurfaceConfiguration, TextureView, VertexState,
};

use crate::{
    entity::Vertex,
    layouts::Layouts,
    render_world::{DrawWorld, ExtractedCamera, RenderWorld},
    texture::{Texture, TextureType},
};

use super::RenderPass;

pub struct ModelPass {
    render_pipeline: RenderPipeline,
    depth_texture: Texture,
}

impl ModelPass {
    pub fn new(device: &Device, config: &SurfaceConfiguration, layouts: &Layouts) -> ModelPass {
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Shader"),
            source: ShaderSource::Wgsl(include_str!("../shaders/model.wgsl").into()),
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
                &layouts.camera,
                &layouts.transform,
                &layouts.material,
                &layouts.light,
            ],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
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
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
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
        }
    }
}

impl RenderPass for ModelPass {
    fn draw(
        &self,
        device: &Device,
        queue: &Queue,
        view: &TextureView,
        world: &RenderWorld,
        camera: &ExtractedCamera,
    ) {
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Model render Encoder"),
        });

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Model render Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Load,
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: &self.depth_texture.view,
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(1.0),
                    store: StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, camera, &[]);
        render_pass.set_bind_group(3, &world.lights_bind_group, &[]);

        render_pass.draw_world(world);

        drop(render_pass);
        let encoder = encoder.finish();

        queue.submit(std::iter::once(encoder));
    }

    fn resize(&mut self, device: &Device, width: u32, height: u32) {
        self.depth_texture = Texture::new(
            device,
            width,
            height,
            Some("Depth texture"),
            TextureType::Depth,
        );
    }
}
