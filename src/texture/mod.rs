mod cubemap;

use wgpu::{
    Device, Extent3d, ImageCopyTextureBase, ImageDataLayout, Origin3d, Queue, Sampler,
    TextureDescriptor, TextureView,
};

pub use cubemap::CubeMap;

pub enum TextureType {
    Diffuse,
    Normal,
    Depth,
}

#[derive(Debug)]
pub struct Texture {
    texture: wgpu::Texture,
    pub view: TextureView,
    pub sampler: Sampler,
}

impl Texture {
    pub const DIFFUSE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;
    pub const NORMAL_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn new(
        device: &Device,
        width: u32,
        height: u32,
        label: Option<&str>,
        texture_type: TextureType,
    ) -> Texture {
        let size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let (format, usage) = match texture_type {
            TextureType::Diffuse => (
                Texture::DIFFUSE_FORMAT,
                wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            ),
            TextureType::Normal => (
                Texture::NORMAL_FORMAT,
                wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            ),
            TextureType::Depth => (
                Texture::DEPTH_FORMAT,
                wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
            ),
        };

        let texture = device.create_texture(&TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Texture {
            texture,
            view,
            sampler,
        }
    }

    pub fn write(&self, queue: &Queue, data: &[u8]) {
        queue.write_texture(
            ImageCopyTextureBase {
                texture: &self.texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            data,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * self.texture.width()),
                rows_per_image: Some(self.texture.height()),
            },
            self.texture.size(),
        );
    }

    pub fn init(
        device: &Device,
        queue: &Queue,
        width: u32,
        height: u32,
        data: &[u8],
        label: Option<&str>,
        texture_type: TextureType,
    ) -> Texture {
        let texture = Texture::new(device, width, height, label, texture_type);
        texture.write(queue, data);

        texture
    }
}
