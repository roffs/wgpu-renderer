mod mesh;

use wgpu::RenderPass;

pub use self::mesh::{DrawMesh, Mesh, Vertex};
use crate::material::Material;

pub struct Model {
    pub meshes: Vec<(Mesh, usize)>,
    pub materials: Vec<Material>,
}

impl Model {
    pub fn new(meshes: Vec<(Mesh, usize)>, materials: Vec<Material>) -> Model {
        Model { meshes, materials }
    }
}

pub trait DrawModel<'a> {
    fn draw_model(&mut self, model: &'a Model);
}

impl<'a> DrawModel<'a> for RenderPass<'a> {
    fn draw_model(&mut self, model: &'a Model) {
        for (mesh, material_index) in &model.meshes {
            let material = &model.materials[*material_index];
            self.draw_mesh(mesh, material);
        }
    }
}
