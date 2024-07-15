use crate::{entity::Entity, light::PointLight, skybox::Skybox};

pub struct Scene {
    pub entities: Vec<Entity>,
    pub lights: Vec<PointLight>,
    pub skybox: Skybox,
}
