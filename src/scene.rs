use crate::{entity::Entity, light::PointLight, texture::CubeMap};

pub struct Scene {
    pub entities: Vec<Entity>,
    pub lights: Vec<PointLight>,
    pub env_map: CubeMap,
}
