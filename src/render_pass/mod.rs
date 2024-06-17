mod model_pass;
mod shadow_pass;
mod skybox_pass;

pub use self::{model_pass::ModelPass, shadow_pass::ShadowPass, skybox_pass::SkyboxPass};
use wgpu::TextureView;

use crate::{camera::Camera, scene::Scene};

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
