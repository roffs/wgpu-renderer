mod geometry;
mod mesh;
mod node;
mod vertex;

use std::fmt::Debug;

pub use self::{geometry::*, mesh::Mesh, node::*, vertex::Vertex};

use crate::{material::Material, transform::Transform};

pub struct Entity {
    nodes: Vec<Node>,
    materials: Vec<Material>,
    pub transform: Transform,
}

impl Debug for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Entity")
            .field("nodes", &self.nodes)
            .field("transform", &self.transform)
            .finish()
    }
}

impl Entity {
    pub fn new(nodes: Vec<Node>, materials: Vec<Material>, transform: Transform) -> Entity {
        Entity {
            nodes,
            materials,
            transform,
        }
    }

    pub fn get_materials(&self) -> &Vec<Material> {
        &self.materials
    }

    pub fn get_nodes(&self) -> &Vec<Node> {
        &self.nodes
    }

    pub fn apply_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }
}
