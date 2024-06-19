use wgpu::RenderPass;

use crate::{camera::Camera, material::Material, transform::Transform};

use super::temp_mesh::{DrawMesh, Mesh};

pub struct Entity {
    nodes: Vec<Node>,
    materials: Vec<Material>,
}

impl Entity {
    pub fn new(nodes: Vec<Node>, materials: Vec<Material>) -> Entity {
        Entity { nodes, materials }
    }
}

pub enum Node {
    _Camera {
        camera: Camera,
        transform: Option<Transform>,
        children: Vec<Node>,
    },
    Mesh {
        mesh: Mesh,
        transform: Option<Transform>,
        children: Vec<Node>,
    },
    Empty {
        transform: Option<Transform>,
        children: Vec<Node>,
    },
}

pub trait DrawEntity<'a> {
    fn draw_entity(&mut self, model: &'a Entity);
}

impl<'a> DrawEntity<'a> for RenderPass<'a> {
    fn draw_entity(&mut self, entity: &'a Entity) {
        for node in &entity.nodes {
            self.draw_node(node, &entity.materials);
        }
    }
}

pub trait DrawNode<'a> {
    fn draw_node(&mut self, node: &'a Node, materials: &'a [Material]);
}

impl<'a> DrawNode<'a> for RenderPass<'a> {
    fn draw_node(&mut self, node: &'a Node, materials: &'a [Material]) {
        if let Node::Mesh { mesh, children, .. } = node {
            self.draw_mesh(mesh, materials);

            for child in children {
                self.draw_node(child, materials);
            }
        };

        // TODO: Remove duplication
        if let Node::Empty { children, .. } = node {
            for child in children {
                self.draw_node(child, materials);
            }
        }

        if let Node::_Camera { children, .. } = node {
            for child in children {
                self.draw_node(child, materials);
            }
        }
    }
}
