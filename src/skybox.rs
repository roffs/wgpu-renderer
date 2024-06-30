use crate::{entity::Geometry, texture::CubeMap};

pub struct Skybox {
    pub geometry: Geometry,
    pub cube_map: CubeMap,
}

impl Skybox {
    pub fn new(cube_map: CubeMap) -> Skybox {
        let geometry = Geometry::cube();

        Skybox { geometry, cube_map }
    }
}
