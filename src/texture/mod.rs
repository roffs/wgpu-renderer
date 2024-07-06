mod cubemap;

use wgpu::{
    Device, Extent3d, ImageCopyTextureBase, ImageDataLayout, Origin3d, Queue, Sampler,
    TextureDescriptor, TextureFormat, TextureUsages, TextureView,
};

pub use cubemap::CubeMap;

#[derive(Debug)]
pub struct Texture {
    texture: wgpu::Texture,
    pub view: TextureView,
    pub sampler: Sampler,
    pub format: wgpu::TextureFormat,
}

// TODO: Refactor texture
impl Texture {
    pub const SRGBA_UNORM: TextureFormat = TextureFormat::Rgba8UnormSrgb;
    pub const RGBA_UNORM: TextureFormat = TextureFormat::Rgba8Unorm;
    pub const DEPTH_32_FLOAT: TextureFormat = TextureFormat::Depth32Float;
    pub const RGBA_16_FLOAT: TextureFormat = TextureFormat::Rgba16Float;

    pub fn new(
        device: &Device,
        width: u32,
        height: u32,
        label: Option<&str>,
        format: TextureFormat,
        usage: TextureUsages,
    ) -> Texture {
        let size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
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
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        Texture {
            texture,
            view,
            sampler,
            format,
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
        format: TextureFormat,
    ) -> Texture {
        let texture = Texture::new(
            device,
            width,
            height,
            label,
            format,
            TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
        );
        texture.write(queue, data);

        texture
    }
}
