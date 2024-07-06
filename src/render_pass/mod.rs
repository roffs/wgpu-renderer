mod model_pass;
mod pipeline;
mod shadow_pass;
mod skybox_pass;

use wgpu::{Device, Queue, TextureView};

pub use self::{model_pass::ModelPass, shadow_pass::ShadowPass, skybox_pass::SkyboxPass};

use crate::render_world::{ExtractedCamera, RenderWorld};

pub trait RenderPass {
    fn draw(
        &self,
        device: &Device,
        queue: &Queue,
        view: &TextureView,
        world: &RenderWorld,
        camera: &ExtractedCamera,
    );
    fn resize(&mut self, device: &Device, width: u32, height: u32);
}
