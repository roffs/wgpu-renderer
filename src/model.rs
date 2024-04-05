use crate::{mesh::Mesh, texture::Texture, transform::Transform};

pub struct Model {
    pub meshes: Vec<(Mesh, usize)>,
    pub textures: Vec<Texture>,
    pub transform: Transform,
}

impl Model {
    pub fn new(meshes: Vec<(Mesh, usize)>, textures: Vec<Texture>, transform: Transform) -> Model {
        Model {
            meshes,
            textures,
            transform,
        }
    }
}
