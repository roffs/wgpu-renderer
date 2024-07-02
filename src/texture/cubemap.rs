use wgpu::{
    Device, Extent3d, ImageCopyTextureBase, ImageDataLayout, Origin3d, Queue, Sampler,
    TextureDescriptor, TextureView,
};

pub struct CubeMap {
    pub texture: wgpu::Texture,
    pub view: TextureView,
    pub sampler: Sampler,
}

impl CubeMap {
    pub fn new(device: &Device, width: u32, height: u32, label: Option<&str>) -> CubeMap {
        let texture_size = Extent3d {
            width,
            height,
            depth_or_array_layers: 6,
        };

        let texture = device.create_texture(&TextureDescriptor {
            label,
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::Cube),
            ..Default::default()
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        CubeMap {
            texture,
            view,
            sampler,
        }
    }

    pub fn new_depth_cubemap(
        device: &Device,
        width: u32,
        height: u32,
        label: Option<&str>,
    ) -> CubeMap {
        let texture_size = Extent3d {
            width,
            height,
            depth_or_array_layers: 6,
        };

        let texture = device.create_texture(&TextureDescriptor {
            label,
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::Cube),
            ..Default::default()
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        CubeMap {
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

    pub fn new_with_data(
        device: &Device,
        queue: &Queue,
        width: u32,
        height: u32,
        data: &[u8],
        label: Option<&str>,
    ) -> CubeMap {
        let texture = CubeMap::new(device, width, height, label);
        texture.write(queue, data);

        texture
    }

    pub fn create_face_view(&self, face: usize) -> TextureView {
        let view = self.texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some(format!("CubeMap view from face: {}", face).as_str()),
            dimension: Some(wgpu::TextureViewDimension::D2),
            base_array_layer: face as u32,
            array_layer_count: Some(1),
            ..Default::default()
        });

        view
    }
}
