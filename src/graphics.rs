use wgpu::{
    Buffer, BufferDescriptor, BufferUsages, CompositeAlphaMode, Device, DeviceDescriptor, Features,
    FragmentState, IndexFormat, Instance, InstanceDescriptor, Limits, MultisampleState,
    PipelineLayoutDescriptor, PrimitiveState, Queue, RenderPipeline, RequestAdapterOptions,
    Surface, SurfaceConfiguration, TextureUsages, VertexAttribute, VertexBufferLayout, VertexState,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::vertex::Vertex;

#[repr(C)]
struct BufferElementData {
    color: (f32, f32, f32),
    _layout_offset: u32,
    scale: (f32, f32),
    offset: (f32, f32),
}

pub struct GraphicsContext<'a> {
    device: Device,
    queue: Queue,
    surface: Surface<'a>,
    config: SurfaceConfiguration,
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

impl<'a> GraphicsContext<'a> {
    pub fn new(window: &'a Window) -> GraphicsContext<'a> {
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

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let element_size = std::mem::size_of::<BufferElementData>() as u64;
        let number_of_elements = 100_u64;
        let mut buffer_data: Vec<BufferElementData> = Vec::new();

        for _ in 0..number_of_elements {
            let r = rand::random::<f32>();
            let g = rand::random::<f32>();
            let b = rand::random::<f32>();

            let scale_x = rand::random::<f32>() * 0.5;
            let scale_y = rand::random::<f32>() * 0.5;

            let offset_x = (rand::random::<f32>() - 0.5) * 2.0;
            let offset_y = (rand::random::<f32>() - 0.5) * 2.0;

            let element = BufferElementData {
                color: (r, g, b),
                _layout_offset: 0,
                scale: (scale_x, scale_y),
                offset: (offset_x, offset_y),
            };

            buffer_data.push(element);
        }

        // STORAGE BUFFER

        let buffer_data = unsafe {
            std::slice::from_raw_parts(
                buffer_data.as_slice() as *const [BufferElementData] as *const u8,
                (element_size * number_of_elements) as usize,
            )
        };

        let storage_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Storage buffer"),
            size: element_size * number_of_elements,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        queue.write_buffer(&storage_buffer, 0, buffer_data);

        // VERTEX BUFFER
        let vertex_buffer_data = &[
            Vertex::new((-0.5, -0.5)),
            Vertex::new((0.5, -0.5)),
            Vertex::new((0.5, 0.5)),
            Vertex::new((-0.5, 0.5)),
        ];

        let vertex_buffer_data = unsafe {
            std::slice::from_raw_parts(
                vertex_buffer_data as *const [Vertex] as *const u8,
                (element_size * number_of_elements) as usize,
            )
        };

        let vertex_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Vertex buffer"),
            size: element_size * number_of_elements,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        queue.write_buffer(&vertex_buffer, 0, vertex_buffer_data);

        // INDEX BUFFER

        let index_buffer_data: &[u16] = &[0, 1, 2, 0, 2, 3];

        let index_buffer_data = unsafe {
            std::slice::from_raw_parts(
                index_buffer_data as *const [u16] as *const u8,
                (element_size * number_of_elements) as usize,
            )
        };

        let index_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Index buffer"),
            size: element_size * number_of_elements,
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        queue.write_buffer(&index_buffer, 0, index_buffer_data);

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Pipeline layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[VertexAttribute {
                        format: wgpu::VertexFormat::Float32x2,
                        offset: 0,
                        shader_location: 0,
                    }],
                }],
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
            vertex_buffer,
            index_buffer,
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
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);
        render_pass.draw_indexed(0..6, 0, 0..1);

        drop(render_pass);
        let encoder = encoder.finish();

        self.queue.submit(std::iter::once(encoder));
        output.present();
    }
}
