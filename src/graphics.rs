use wgpu::{
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, CompositeAlphaMode, Device,
    DeviceDescriptor, Features, FragmentState, IndexFormat, Instance, InstanceDescriptor, Limits,
    MultisampleState, PipelineLayoutDescriptor, PrimitiveState, Queue, RenderPipeline,
    RequestAdapterOptions, SamplerBindingType, ShaderStages, Surface, SurfaceConfiguration,
    TextureUsages, VertexState,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{mesh::Mesh, texture::Texture, vertex::Vertex};

pub struct GraphicsContext<'a> {
    device: Device,
    queue: Queue,
    surface: Surface<'a>,
    config: SurfaceConfiguration,
    render_pipeline: RenderPipeline,
    mesh: Mesh,
    texture: Texture,
}

impl<'a> GraphicsContext<'a> {
    pub fn new(window: &'a Window) -> GraphicsContext<'a> {
        let (device, queue, config, surface) = create_graphics_context(window);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let mesh = Mesh::new(&device, &queue);

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
            &device,
            &queue,
            &texture_bind_group_layout,
            "./assets/textures/test.png",
            Some("Test texture"),
        );

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Pipeline layout"),
            bind_group_layouts: &[&texture_bind_group_layout],
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
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        GraphicsContext {
            device,
            queue,
            surface,
            config,
            render_pipeline,
            mesh,
            texture,
        }
    }

    pub fn resize(&mut self, &PhysicalSize { width, height }: &PhysicalSize<u32>) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
        }

        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(&self) {
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
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.mesh.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.mesh.index_buffer.slice(..), IndexFormat::Uint16);
        render_pass.set_bind_group(0, &self.texture, &[]);
        render_pass.draw_indexed(0..6, 0, 0..1);

        drop(render_pass);
        let encoder = encoder.finish();

        self.queue.submit(std::iter::once(encoder));
        output.present();
    }
}

fn create_graphics_context(window: &Window) -> (Device, Queue, SurfaceConfiguration, Surface) {
    let instance = Instance::new(InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let surface = instance.create_surface(window).unwrap();

    let adapter = pollster::block_on(async {
        instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
    })
    .unwrap();

    let (device, queue) = pollster::block_on(async {
        adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("Device"),
                    required_features: Features::empty(),
                    required_limits: Limits::default(),
                },
                None,
            )
            .await
    })
    .unwrap();

    let size = window.inner_size();

    let config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        desired_maximum_frame_latency: 2,
        alpha_mode: CompositeAlphaMode::Auto,
        view_formats: vec![],
    };

    surface.configure(&device, &config);

    (device, queue, config, surface)
}
