use wgpu::{
    CompositeAlphaMode, Device, DeviceDescriptor, Features, Instance, InstanceDescriptor, Limits,
    Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, TextureUsages,
};
use winit::{dpi::PhysicalSize, window::Window};

pub struct GraphicsContext<'a> {
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'a>,
    config: SurfaceConfiguration,
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

        GraphicsContext {
            device,
            queue,
            surface,
            config,
        }
    }

    pub fn resize(&mut self, &PhysicalSize { width, height }: &PhysicalSize<u32>) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
        }

        self.surface.configure(&self.device, &self.config);
    }
}
