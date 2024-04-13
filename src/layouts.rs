use std::collections::HashMap;

use wgpu::{
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferBindingType, Device,
    SamplerBindingType, ShaderStages,
};

#[derive(Eq, PartialEq, Hash)]
pub enum Layout {
    Transform,
    Material,
    Light,
}

pub struct Layouts(HashMap<Layout, wgpu::BindGroupLayout>);

impl Layouts {
    pub fn new(device: &Device) -> Layouts {
        let mut layouts = HashMap::new();

        let transform_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Transform bind group layout"),
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

        let material_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Material bind group layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
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

        let lightp_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Light bind group layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        layouts.insert(Layout::Transform, transform_layout);
        layouts.insert(Layout::Material, material_layout);
        layouts.insert(Layout::Light, lightp_layout);

        Layouts(layouts)
    }

    pub fn get(&self, layout: &Layout) -> &wgpu::BindGroupLayout {
        self.0.get(layout).unwrap()
    }
}