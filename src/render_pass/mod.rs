mod model_render_pass;
mod skybox_render_pass;

use std::collections::HashMap;

pub use model_render_pass::ModelRenderPass;
pub use skybox_render_pass::SkyboxRenderPass;
use wgpu::{Device, Queue, SurfaceConfiguration, TextureView};

use crate::{camera::Camera, layouts::Layouts, scene::Scene};

pub trait RenderPass {
    fn draw(&self, view: &TextureView, camera: &Camera, scene: &Scene);
    fn resize(&mut self, width: u32, height: u32);
}

#[derive(Eq, PartialEq, Hash)]
pub enum PassKind {
    Model,
    Skybox,
}

pub struct RenderPasses<'a>(HashMap<PassKind, Box<dyn RenderPass + 'a>>);

impl<'a> RenderPasses<'a> {
    pub fn new(
        device: &'a Device,
        queue: &'a Queue,
        config: &SurfaceConfiguration,
        layouts: &Layouts,
    ) -> RenderPasses<'a> {
        let model_pass = ModelRenderPass::new(device, queue, config, layouts, 2); //TODO remove hardcoded num. of lights.

        let skybox_render_pass = SkyboxRenderPass::new(device, queue, config, layouts);

        let mut render_passes: HashMap<PassKind, Box<dyn RenderPass>> = HashMap::new();

        render_passes.insert(PassKind::Model, Box::new(model_pass));
        render_passes.insert(PassKind::Skybox, Box::new(skybox_render_pass));

        RenderPasses(render_passes)
    }

    #[allow(clippy::borrowed_box)]
    pub fn get(&self, kind: &PassKind) -> &Box<dyn RenderPass + 'a> {
        self.0.get(kind).unwrap()
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        for pass in self.0.values_mut() {
            pass.resize(width, height);
        }
    }
}
