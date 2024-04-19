use wgpu::{
    DeviceDescriptor, Features, Instance, InstanceDescriptor, Limits, RequestAdapterOptions,
};

use crate::surface_context::SurfaceContext;

pub struct GpuContext {
    pub instance: wgpu::Instance,
    _adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl GpuContext {
    pub fn new(surface: &SurfaceContext) -> GpuContext {
        let instance = Instance::new(InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter = pollster::block_on(async {
            instance
                .request_adapter(&RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    compatible_surface: surface.get(),
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

        GpuContext {
            instance,
            _adapter: adapter,
            device,
            queue,
        }
    }
}
