use wgpu::{
    CommandEncoderDescriptor, Device, LoadOp, Operations, PipelineLayoutDescriptor, Queue,
    RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor,
    RenderPipeline, ShaderModuleDescriptor, ShaderSource, StoreOp, SurfaceConfiguration,
    TextureView,
};

use crate::{
    entity::Vertex,
    layouts::Layouts,
    render_world::{DrawWorld, ExtractedCamera, RenderWorld},
    texture::{Texture, TextureType},
};

use super::pipeline::create_pipeline;

pub struct ModelPass {
    pipeline: RenderPipeline,
    depth_texture: Texture,
}

impl ModelPass {
    pub fn new(device: &Device, config: &SurfaceConfiguration, layouts: &Layouts) -> ModelPass {
        let shader = ShaderModuleDescriptor {
            label: Some("Shader"),
            source: ShaderSource::Wgsl(include_str!("../shaders/model.wgsl").into()),
        };

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

        let pipeline = create_pipeline(
            device,
            &pipeline_layout,
            &[Vertex::desc()],
            config.format,
            Some(Texture::DEPTH_FORMAT),
            shader,
        );

        ModelPass {
            pipeline,
            depth_texture,
        }
    }

    pub fn draw(
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

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, camera, &[]);
        render_pass.set_bind_group(3, &world.lights_bind_group, &[]);

        render_pass.draw_world(world);

        drop(render_pass);
        let encoder = encoder.finish();

        queue.submit(std::iter::once(encoder));
    }

    pub fn resize(&mut self, device: &Device, width: u32, height: u32) {
        self.depth_texture = Texture::new(
            device,
            width,
            height,
            Some("Depth texture"),
            TextureType::Depth,
        );
    }
}
