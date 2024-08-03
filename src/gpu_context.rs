use wgpu::{
    DeviceDescriptor, Features, Instance, InstanceDescriptor, Limits, MemoryHints,
    RequestAdapterOptions,
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
                        required_features: Features::TEXTURE_BINDING_ARRAY | Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING | Features::DEPTH_CLIP_CONTROL | Features::PUSH_CONSTANTS,
                        required_limits: Limits{
                            max_push_constant_size: 4,
                            max_bind_groups: 5,
                            ..Default::default()
                        },
                        memory_hints: MemoryHints::default()
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
