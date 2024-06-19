mod geometry;
mod mesh;
mod node;
mod vertex;

pub use self::{geometry::*, mesh::Mesh, node::*, vertex::Vertex};

use node::DrawNode;

use wgpu::RenderPass;

use crate::material::Material;

pub struct Entity {
    nodes: Vec<Box<dyn Node>>,
    materials: Vec<Material>,
}

impl Entity {
    pub fn new(nodes: Vec<Box<dyn Node>>, materials: Vec<Material>) -> Entity {
        Entity { nodes, materials }
    }
}

pub trait DrawEntity<'a> {
    fn draw_entity(&mut self, model: &'a Entity);
}

impl<'a> DrawEntity<'a> for RenderPass<'a> {
    fn draw_entity(&mut self, entity: &'a Entity) {
        for node in &entity.nodes {
            self.draw_node(node.as_ref(), &entity.materials);
        }
    }
}
