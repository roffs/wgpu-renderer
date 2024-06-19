use crate::{entity::Mesh, transform::Transform};

use super::Node;

pub struct EmptyNode {
    pub transform: Option<Transform>,
    pub children: Vec<Box<dyn Node>>,
}

impl Node for EmptyNode {
    fn get_children(&self) -> Vec<&dyn Node> {
        self.children.iter().map(|c| c.as_ref()).collect()
    }

    fn get_mesh(&self) -> Option<&Mesh> {
        None
    }
}
