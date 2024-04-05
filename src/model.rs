use wgpu::RenderPass;

use crate::{
    mesh::{DrawMesh, Mesh},
    texture::Texture,
    transform::Transform,
};

pub struct Model {
    pub meshes: Vec<(Mesh, usize)>,
    pub textures: Vec<Texture>,
    pub transform: Transform,
}

impl Model {
    pub fn new(meshes: Vec<(Mesh, usize)>, textures: Vec<Texture>, transform: Transform) -> Model {
        Model {
            meshes,
            textures,
            transform,
        }
    }
}

pub trait DrawModel<'a> {
    fn draw_model(&mut self, model: &'a Model);
}

impl<'a> DrawModel<'a> for RenderPass<'a> {
    fn draw_model(&mut self, model: &'a Model) {
        self.set_bind_group(1, &model.transform, &[]);

        for (mesh, texture_index) in &model.meshes {
            let texture = &model.textures[*texture_index];
            self.draw_mesh(mesh, texture);
        }
    }
}
