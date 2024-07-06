use wgpu::{
    Color, CommandEncoderDescriptor, Device, LoadOp, Operations, PipelineLayoutDescriptor, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, ShaderModuleDescriptor,
    ShaderSource, StoreOp, SurfaceConfiguration, TextureView,
};

use crate::{
    layouts::Layouts,
    render_world::{DrawWorld, ExtractedCamera, RenderWorld},
};

use super::pipeline::create_pipeline;

pub struct SkyboxPass {
    pipeline: RenderPipeline,
}

impl SkyboxPass {
    pub fn new(device: &Device, config: &SurfaceConfiguration, layouts: &Layouts) -> SkyboxPass {
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Shader"),
            source: ShaderSource::Wgsl(include_str!("../shaders/skybox.wgsl").into()),
        });

        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Skybox pipeline layout"),
            bind_group_layouts: &[&layouts.camera, &layouts.cube_map],
            push_constant_ranges: &[],
        });

        let pipeline = create_pipeline(device, &layout, &[], config.format, None, &shader);

        SkyboxPass { pipeline }
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

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, camera, &[]);
        render_pass.draw_skybox(world);

        drop(render_pass);
        let encoder = encoder.finish();

        queue.submit(std::iter::once(encoder));
    }
}
