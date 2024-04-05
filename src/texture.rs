use wgpu::{BindGroup, Sampler, TextureView};

pub struct Texture {
    pub view: TextureView,
    pub sampler: Sampler,
    pub bind_group: Option<BindGroup>,
}

impl Texture {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn new(view: TextureView, sampler: Sampler, bind_group: Option<BindGroup>) -> Texture {
        Texture {
            view,
            sampler,
            bind_group,
        }
    }
}
