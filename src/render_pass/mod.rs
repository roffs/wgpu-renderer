mod model_pass;
mod shadow_pass;
mod skybox_pass;

use std::collections::HashMap;

pub use model_pass::ModelPass;
pub use skybox_pass::SkyboxPass;
use wgpu::{Device, SurfaceConfiguration, TextureView};

use crate::{camera::Camera, layouts::Layouts, scene::Scene};

use self::shadow_pass::ShadowPass;

pub trait RenderPass {
    fn draw(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        view: &TextureView,
        camera: &Camera,
        scene: &Scene,
    );
    fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32);
}

#[derive(Eq, PartialEq, Hash)]
pub enum PassKind {
    Model,
    Skybox,
    Shadow,
}

pub struct RenderPasses(HashMap<PassKind, Box<dyn RenderPass>>);

impl RenderPasses {
    pub fn new(
        device: &Device,
        config: &SurfaceConfiguration,
        layouts: &Layouts,
        scene: &Scene,
    ) -> RenderPasses {
        let mut render_passes: HashMap<PassKind, Box<dyn RenderPass>> = HashMap::new();

        let model_pass = ModelPass::new(device, config, layouts, &scene.lights);
        let skybox_pass = SkyboxPass::new(device, config, layouts);
        let shadow_pass = ShadowPass::new(device, config, layouts, &scene.lights);

        render_passes.insert(PassKind::Model, Box::new(model_pass));
        render_passes.insert(PassKind::Skybox, Box::new(skybox_pass));
        render_passes.insert(PassKind::Shadow, Box::new(shadow_pass));

        RenderPasses(render_passes)
    }

    #[allow(clippy::borrowed_box)]
    pub fn get(&self, kind: &PassKind) -> &Box<dyn RenderPass> {
        self.0.get(kind).unwrap()
    }

    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        for pass in self.0.values_mut() {
            pass.resize(device, width, height);
        }
    }
}
