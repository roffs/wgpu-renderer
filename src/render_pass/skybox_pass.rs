use cgmath::Matrix;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, Buffer, BufferDescriptor, BufferUsages, Device,
    FragmentState, MultisampleState, PipelineLayoutDescriptor, PrimitiveState, RenderPipeline,
    SurfaceConfiguration, TextureView, VertexState,
};

use crate::{
    camera::Camera,
    layouts::{Layout, Layouts},
    model::Vertex,
    scene::Scene,
    skybox::DrawSkybox,
};

use super::RenderPass;

pub struct SkyboxPass {
    render_pipeline: RenderPipeline,
    camera_buffer: Buffer,
    camera_bind_group: BindGroup,
}

impl SkyboxPass {
    pub fn new(device: &Device, config: &SurfaceConfiguration, layouts: &Layouts) -> SkyboxPass {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/skybox.wgsl").into()),
        });

        let camera_buffer_size = std::mem::size_of::<cgmath::Matrix4<f32>>();

        let camera_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Skybox camera buffer"),
            size: camera_buffer_size as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let camera_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Skybox camera bind group"),
            layout: layouts.get(&Layout::Transform),
            entries: &[BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        // PIPELINE
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Skybox pipeline layout"),
            bind_group_layouts: &[
                layouts.get(&Layout::Transform),
                layouts.get(&Layout::Skybox),
            ],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Skybox render pipeline"),
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
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
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

        SkyboxPass {
            render_pipeline,
            camera_buffer,
            camera_bind_group,
        }
    }
}

impl RenderPass for SkyboxPass {
    fn draw(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        view: &TextureView,
        camera: &Camera,
        scene: &Scene,
    ) {
        let view_projection = camera.get_projection() * camera.get_rotation();
        let view_projection = unsafe {
            std::slice::from_raw_parts(
                view_projection.as_ptr() as *const u8,
                std::mem::size_of::<cgmath::Matrix4<f32>>(),
            )
        };

        queue.write_buffer(&self.camera_buffer, 0, view_projection);

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

        render_pass.draw_skybox(&scene.skybox);

        drop(render_pass);
        let encoder = encoder.finish();

        queue.submit(std::iter::once(encoder));
    }

    fn resize(&mut self, _device: &wgpu::Device, _width: u32, _height: u32) {}
}
