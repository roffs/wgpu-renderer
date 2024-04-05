use crate::texture::Texture;

pub struct Material {
    pub diffuse: Texture,
}

impl Material {
    pub fn new(diffuse: Texture) -> Material {
        Material { diffuse }
    }
}
