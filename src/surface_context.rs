use std::sync::Arc;

use wgpu::{CompositeAlphaMode, SurfaceConfiguration, TextureUsages};
use winit::window::Window;

use crate::GpuContext;

pub struct SurfaceContext {
    surface: Option<wgpu::Surface<'static>>,
    config: Option<wgpu::SurfaceConfiguration>,
}

impl SurfaceContext {
    pub fn new() -> Self {
        Self {
            surface: None,
            config: None,
        }
    }
    pub fn init(&mut self, context: &GpuContext, window: Arc<Window>) {
        let size = window.inner_size();

        let surface = context.instance.create_surface(window).unwrap();

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

        self.surface = Some(surface);
        self.config = Some(config);
    }

    pub fn get(&self) -> Option<&wgpu::Surface> {
        self.surface.as_ref()
    }

    pub fn config(&self) -> &wgpu::SurfaceConfiguration {
        self.config.as_ref().unwrap()
    }

    pub fn configure(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        let config = self.config.as_mut().unwrap();
        config.width = width;
        config.height = height;

        self.surface.as_ref().unwrap().configure(device, config);
    }
}
