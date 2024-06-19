use wgpu::RenderPass;

use crate::{material::Material, transform::Transform};

use super::{mesh::DrawMesh, Mesh};

pub struct Node {
    pub transform: Option<Transform>,
    pub children: Vec<Node>,
    pub mesh: Option<Mesh>,
}

pub trait DrawNode<'a> {
    fn draw_node(&mut self, node: &'a Node, materials: &'a [Material]);
}

impl<'a> DrawNode<'a> for RenderPass<'a> {
    fn draw_node(&mut self, node: &'a Node, materials: &'a [Material]) {
        if let Some(mesh) = &node.mesh {
            self.draw_mesh(mesh, materials);
        }

        for child in &node.children {
            self.draw_node(child, materials);
        }
    }
}
