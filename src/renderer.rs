use cgmath::{Deg, Matrix, Matrix4};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, BufferDescriptor, BufferUsages, DepthBiasState,
    DepthStencilState, Device, FragmentState, IndexFormat, MultisampleState, Operations,
    PipelineLayoutDescriptor, PrimitiveState, Queue, RenderPassDepthStencilAttachment,
    RenderPipeline, SamplerBindingType, ShaderStages, StencilState, Surface, SurfaceConfiguration,
    VertexState,
};

use crate::{camera::Camera, mesh::Mesh, texture::Texture, vertex::Vertex};

pub struct Renderer<'a> {
    device: &'a Device,
    queue: &'a Queue,
    surface: &'a Surface<'a>,
    config: &'a mut SurfaceConfiguration,
    render_pipeline: RenderPipeline,
    mesh: Mesh,
    depth_texture: Texture,
    texture_bind_group: BindGroup,
    model_bind_group: BindGroup,
    camera_bind_group_layout: BindGroupLayout,
}

impl<'a> Renderer<'a> {
    pub fn new(
        device: &'a Device,
        queue: &'a Queue,
        surface: &'a Surface,
        config: &'a mut SurfaceConfiguration,
    ) -> Renderer<'a> {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let mesh = Mesh::cube(device, queue);

        // TEXTURE

        let texture_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Texture bind group layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering),
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                ],
            });

        let texture = Texture::new(
            device,
            queue,
            "./assets/textures/test.png",
            Some("Test texture"),
        );

        let texture_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Texture bind group"),
            layout: &texture_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
            ],
        });

        // MODEL

        let model_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Model bind group layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let model_translation =
            cgmath::Matrix4::from_translation(cgmath::Vector3::<f32>::new(0.2, 0.0, 0.0));
        let model_rotation = cgmath::Matrix4::<f32>::from_angle_z(Deg(15.0));
        let model_scale = cgmath::Matrix4::<f32>::from_scale(0.5);

        let model_buffer_data = model_translation * model_rotation * model_scale;

        let model_buffer_size = std::mem::size_of::<cgmath::Matrix4<f32>>();
        let model_buffer_data = unsafe {
            std::slice::from_raw_parts(model_buffer_data.as_ptr() as *const u8, model_buffer_size)
        };

        let model_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Model buffer"),
            size: std::mem::size_of::<Matrix4<f32>>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        queue.write_buffer(&model_buffer, 0, model_buffer_data);

        let model_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Model bind group"),
            layout: &model_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: model_buffer.as_entire_binding(),
            }],
        });

        // CAMERA

        let camera_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Model bind group layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::VERTEX,
                        ty: BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::VERTEX,
                        ty: BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });

        // DEPTH TEXTURE
        let depth_texture =
            Texture::new_depth_texture(device, config.width, config.height, Some("Depth texture"));

        // PIPELINE

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Pipeline layout"),
            bind_group_layouts: &[
                &camera_bind_group_layout,
                &model_bind_group_layout,
                &texture_bind_group_layout,
            ],
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

        Renderer {
            device,
            queue,
            surface,
            config,
            render_pipeline,
            mesh,
            depth_texture,
            texture_bind_group,
            model_bind_group,
            camera_bind_group_layout,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
        }

        self.surface.configure(self.device, self.config);
    }

    pub fn render(&self, camera: &Camera) {
        let view_buffer_data = camera.get_view();
        let view_buffer_size = std::mem::size_of::<cgmath::Matrix4<f32>>();
        let view_buffer_data = unsafe {
            std::slice::from_raw_parts(view_buffer_data.as_ptr() as *const u8, view_buffer_size)
        };

        let view_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("View buffer"),
            size: view_buffer_size as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        self.queue.write_buffer(&view_buffer, 0, view_buffer_data);

        let projection_buffer_data = camera.get_projection();
        let projection_buffer_size = std::mem::size_of::<cgmath::Matrix4<f32>>();
        let projection_buffer_data = unsafe {
            std::slice::from_raw_parts(
                projection_buffer_data.as_ptr() as *const u8,
                projection_buffer_size,
            )
        };

        let projection_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("Projection buffer"),
            size: projection_buffer_size as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        self.queue
            .write_buffer(&projection_buffer, 0, projection_buffer_data);

        let camera_bind_group = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("Camera bind group"),
            layout: &self.camera_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: view_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: projection_buffer.as_entire_binding(),
                },
            ],
        });
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
        render_pass.set_vertex_buffer(0, self.mesh.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.mesh.index_buffer.slice(..), IndexFormat::Uint16);
        render_pass.set_bind_group(0, &camera_bind_group, &[]);
        render_pass.set_bind_group(1, &self.model_bind_group, &[]);
        render_pass.set_bind_group(2, &self.texture_bind_group, &[]);
        render_pass.draw_indexed(0..self.mesh.indices_len, 0, 0..1);

        drop(render_pass);
        let encoder = encoder.finish();

        self.queue.submit(std::iter::once(encoder));
        output.present();
    }
}
