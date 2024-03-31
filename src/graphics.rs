use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BufferDescriptor, BufferUsages, CompositeAlphaMode, Device,
    DeviceDescriptor, Features, FragmentState, Instance, InstanceDescriptor, Limits,
    MultisampleState, PipelineLayoutDescriptor, PrimitiveState, Queue, RenderPipeline,
    RequestAdapterOptions, ShaderStages, Surface, SurfaceConfiguration, TextureUsages, VertexState,
};
use winit::{dpi::PhysicalSize, window::Window};

#[repr(C)]
struct UniformData {
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
    uniform_bind_groups: Vec<BindGroup>,
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

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Bind group layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let mut uniform_bind_groups = Vec::new();

        let size = std::mem::size_of::<UniformData>() as u64;

        println!("Buffer size: {}", size);

        for _ in 0..100 {
            let uniform_buffer = device.create_buffer(&BufferDescriptor {
                label: Some("Uniform buffer descriptor"),
                size,
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

            let r = rand::random::<f32>();
            let g = rand::random::<f32>();
            let b = rand::random::<f32>();

            let scale_x = rand::random::<f32>() * 0.5;
            let scale_y = rand::random::<f32>() * 0.5;

            let offset_x = (rand::random::<f32>() - 0.5) * 2.0;
            let offset_y = (rand::random::<f32>() - 0.5) * 2.0;

            let buffer_data = UniformData {
                color: (r, g, b),
                _layout_offset: 0,
                scale: (scale_x, scale_y),
                offset: (offset_x, offset_y),
            };

            let buffer_data = unsafe {
                std::slice::from_raw_parts(
                    &buffer_data as *const UniformData as *const u8,
                    std::mem::size_of::<UniformData>(),
                )
            };

            queue.write_buffer(&uniform_buffer, 0, buffer_data);

            let uniform_bind_group = device.create_bind_group(&BindGroupDescriptor {
                label: Some("Bind group"),
                layout: &uniform_bind_group_layout,
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }],
            });

            uniform_bind_groups.push(uniform_bind_group);
        }

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Pipeline layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
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
            // uniform_buffer,
            uniform_bind_groups,
            // uniform_bind_group_layout,
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

        for uniform_bind_group in &self.uniform_bind_groups {
            render_pass.set_bind_group(0, uniform_bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }

        drop(render_pass);
        let encoder = encoder.finish();

        self.queue.submit(std::iter::once(encoder));
        output.present();
    }
}
