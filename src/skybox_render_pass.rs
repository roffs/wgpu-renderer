use cgmath::Matrix;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Buffer, BufferDescriptor,
    BufferUsages, Device, FragmentState, IndexFormat, MultisampleState, PipelineLayoutDescriptor,
    PrimitiveState, Queue, RenderPipeline, SurfaceConfiguration, TextureView, VertexState,
};

use crate::{
    camera::Camera,
    model::{Mesh, Vertex},
    texture::CubeMap,
};

pub struct SkyboxRenderPass<'a> {
    device: &'a Device,
    queue: &'a Queue,
    render_pipeline: RenderPipeline,
    geometry: Mesh,
    camera_buffer: Buffer,
    camera_bind_group: BindGroup,
}

impl<'a> SkyboxRenderPass<'a> {
    pub fn new(
        device: &'a Device,
        queue: &'a Queue,
        config: &SurfaceConfiguration,
        camera_bind_group_layout: &BindGroupLayout,
        cubemap_bind_group_layout: &BindGroupLayout,
    ) -> SkyboxRenderPass<'a> {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/skybox.wgsl").into()),
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
            layout: camera_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        // PIPELINE
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Skybox pipeline layout"),
            bind_group_layouts: &[camera_bind_group_layout, cubemap_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Skybox render pipeline"),
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

        let geometry = Mesh::cube(device, queue);

        SkyboxRenderPass {
            device,
            queue,
            render_pipeline,
            geometry,
            camera_buffer,
            camera_bind_group,
        }
    }

    pub fn draw(&self, view: &TextureView, skybox: &CubeMap, camera: &Camera) {
        let view_projection = camera.get_projection() * camera.get_rotation();
        let view_projection = unsafe {
            std::slice::from_raw_parts(
                view_projection.as_ptr() as *const u8,
                std::mem::size_of::<cgmath::Matrix4<f32>>(),
            )
        };

        self.queue
            .write_buffer(&self.camera_buffer, 0, view_projection);

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
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

        render_pass.set_vertex_buffer(0, self.geometry.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.geometry.index_buffer.slice(..), IndexFormat::Uint16);
        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
        render_pass.set_bind_group(1, skybox.bind_group.as_ref().unwrap(), &[]);
        render_pass.draw_indexed(0..self.geometry.indices_len, 0, 0..1);

        drop(render_pass);
        let encoder = encoder.finish();

        self.queue.submit(std::iter::once(encoder));
    }
}
