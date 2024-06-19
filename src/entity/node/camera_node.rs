use crate::{camera::Camera, entity::Mesh, transform::Transform};

use super::Node;

pub struct CameraNode {
    camera: Camera,
    transform: Option<Transform>,
    children: Vec<Box<dyn Node>>,
}

impl Node for CameraNode {
    fn get_children(&self) -> Vec<&dyn Node> {
        self.children.iter().map(|c| c.as_ref()).collect()
    }

    fn get_mesh(&self) -> Option<&Mesh> {
        None
    }
}
