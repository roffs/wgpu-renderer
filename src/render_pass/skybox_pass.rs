use wgpu::{
    BlendState, Color, ColorTargetState, ColorWrites, CommandEncoderDescriptor, Device,
    FragmentState, FrontFace, LoadOp, MultisampleState, Operations, PipelineLayoutDescriptor,
    PolygonMode, PrimitiveState, PrimitiveTopology, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor,
    ShaderSource, StoreOp, SurfaceConfiguration, TextureView, VertexState,
};

use crate::{
    layouts::Layouts,
    render_world::{DrawWorld, ExtractedCamera, RenderWorld},
};

use super::RenderPass;

pub struct SkyboxPass {
    render_pipeline: RenderPipeline,
}

impl SkyboxPass {
    pub fn new(device: &Device, config: &SurfaceConfiguration, layouts: &Layouts) -> SkyboxPass {
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Shader"),
            source: ShaderSource::Wgsl(include_str!("../shaders/skybox.wgsl").into()),
        });

        // PIPELINE
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Skybox pipeline layout"),
            bind_group_layouts: &[&layouts.camera, &layouts.sky],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Skybox render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: &[],
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
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        SkyboxPass { render_pipeline }
    }
}

impl RenderPass for SkyboxPass {
    fn draw(
        &self,
        device: &Device,
        queue: &Queue,
        view: &TextureView,
        world: &RenderWorld,
        camera: &ExtractedCamera,
    ) {
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Skybox render Encoder"),
        });

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Skybox render Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::WHITE),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, camera, &[]);
        render_pass.draw_skybox(world);

        drop(render_pass);
        let encoder = encoder.finish();

        queue.submit(std::iter::once(encoder));
    }

    fn resize(&mut self, _device: &Device, _width: u32, _height: u32) {}
}
