use wgpu::RenderPass;

use crate::{
    mesh::{DrawMesh, Mesh},
    texture::Texture,
};

pub struct Model {
    pub meshes: Vec<(Mesh, usize)>,
    pub textures: Vec<Texture>,
}

impl Model {
    pub fn new(meshes: Vec<(Mesh, usize)>, textures: Vec<Texture>) -> Model {
        Model { meshes, textures }
    }
}

pub trait DrawModel<'a> {
    fn draw_model(&mut self, model: &'a Model);
}

impl<'a> DrawModel<'a> for RenderPass<'a> {
    fn draw_model(&mut self, model: &'a Model) {
        for (mesh, texture_index) in &model.meshes {
            let texture = &model.textures[*texture_index];
            self.draw_mesh(mesh, texture);
        }
    }
}
