use crate::transform::Transform;

use std::fmt::Debug;

use super::Mesh;

pub struct Node {
    pub transform: Transform,
    pub children: Vec<Node>,
    pub mesh: Option<Mesh>,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("transform", &self.transform)
            .field("children", &self.children)
            .finish()
    }
}
