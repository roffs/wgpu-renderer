use std::path::Path;

use image::io::Reader;
use wgpu::{Device, Queue, TextureFormat};

use crate::texture::{CubeMap, Texture};

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

    pub fn load_cube_map(device: &Device, queue: &Queue, paths: [&Path; 6]) -> CubeMap {
        let mut data = Vec::new();

        let mut width = 0_u32;
        let mut height = 0_u32;

        for path in paths {
            let image = Reader::open(path).unwrap().decode().unwrap().to_rgba8();

            width = image.width();
            height = image.height();

            data.append(&mut image.into_raw());
        }

        CubeMap::new_with_data(device, queue, width, height, &data, Some("Cubemap texture"))
    }
}
