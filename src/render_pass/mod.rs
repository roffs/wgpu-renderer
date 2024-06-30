mod model_pass;
mod shadow_pass;
mod skybox_pass;

pub use self::{model_pass::ModelPass, shadow_pass::ShadowPass, skybox_pass::SkyboxPass};
use wgpu::TextureView;

use crate::render_world::RenderWorld;

pub trait RenderPass {
    fn draw(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        view: &TextureView,
        world: &RenderWorld,
    );
    fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32);
}
