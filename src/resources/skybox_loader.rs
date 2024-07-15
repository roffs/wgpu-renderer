use std::path::Path;

use wgpu::{Device, Queue};

use crate::{layouts::Layouts, skybox::Skybox};

use super::{irr_map_generator::IrrMapGenerator, HdrLoader, Resources};

pub struct SkyboxLoader {
    hdr_loader: HdrLoader,
    irr_generator: IrrMapGenerator,
}

impl SkyboxLoader {
    pub fn new(device: &Device, layouts: &Layouts) -> SkyboxLoader {
        let hdr_loader = HdrLoader::new(device);
        let irr_generator = IrrMapGenerator::new(device, layouts);

        SkyboxLoader {
            hdr_loader,
            irr_generator,
        }
    }

    pub fn load(
        &self,
        device: &Device,
        queue: &Queue,
        path: &Path,
        dst_size: u32,
        layouts: &Layouts,
    ) -> Skybox {
        let hdr_texture = Resources::load_hdr_texture(device, queue, path);

        let env_map = self.hdr_loader.load(device, queue, &hdr_texture, dst_size);
        let irr_map = self
            .irr_generator
            .generate(device, queue, &env_map, dst_size, layouts);

        Skybox { env_map, irr_map }
    }
}
