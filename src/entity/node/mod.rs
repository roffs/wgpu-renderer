mod camera_node;
mod empty_node;
mod mesh_node;

use wgpu::RenderPass;

pub use self::{empty_node::EmptyNode, mesh_node::MeshNode};

use crate::material::Material;

use super::{mesh::DrawMesh, Mesh};

pub trait Node {
    fn get_children(&self) -> Vec<&dyn Node>;
    fn get_mesh(&self) -> Option<&Mesh>;
}

pub trait DrawNode<'a> {
    fn draw_node(&mut self, node: &'a dyn Node, materials: &'a [Material]);
}

impl<'a> DrawNode<'a> for RenderPass<'a> {
    fn draw_node(&mut self, node: &'a dyn Node, materials: &'a [Material]) {
        if let Some(mesh) = node.get_mesh() {
            self.draw_mesh(mesh, materials);
        }

        for child in node.get_children() {
            self.draw_node(child, materials);
        }
    }
}
