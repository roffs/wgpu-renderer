use std::path::Path;

use image::io::Reader;
use wgpu::{Device, Queue, TextureFormat};

use crate::texture::Texture;

use super::Resources;

impl Resources {
    pub fn load_texture(
        device: &Device,
        queue: &Queue,
        path: &Path,
        format: TextureFormat,
    ) -> Texture {
        let image = Reader::open(path).unwrap().decode().unwrap();

        let width = image.width();
        let height = image.height();

        let data = image.to_rgba8();

        let label = format!("{}", path.display());

        Texture::init(
            device,
            queue,
            width,
            height,
            &data,
            Some(label.as_str()),
            format,
        )
    }

    pub fn load_hdr_texture(device: &Device, queue: &Queue, path: &Path) -> Texture {
        let image = Reader::open(path).unwrap().decode().unwrap();

        let width = image.width();
        let height = image.height();

        let data = image.to_rgba32f().to_vec();

        let data = unsafe {
            let len = data.len() * std::mem::size_of::<f32>();
            let ptr = data.as_ptr() as *const u8;
            std::slice::from_raw_parts(ptr, len)
        };

        let label = format!("{}", path.display());

        Texture::init_hdr(
            device,
            queue,
            width,
            height,
            data,
            Some(label.as_str()),
            Texture::RGBA_32_FLOAT,
        )
    }
}
