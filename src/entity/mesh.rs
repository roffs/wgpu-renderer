use wgpu::RenderPass;

use crate::{
    entity::{DrawGeometry, Geometry},
    material::Material,
};

pub struct Mesh {
    pub primitives: Vec<(Geometry, usize)>,
}

pub trait DrawMesh<'a> {
    fn draw_mesh(&mut self, mesh: &'a Mesh, materials: &'a [Material]);
}

impl<'a> DrawMesh<'a> for RenderPass<'a> {
    fn draw_mesh(&mut self, mesh: &'a Mesh, materials: &'a [Material]) {
        for (geometry, material_index) in &mesh.primitives {
            let material = &materials[*material_index];
            self.draw_geometry(geometry, material);
        }
    }
}
