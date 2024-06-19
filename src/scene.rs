use crate::{entity::Entity, light::PointLight, skybox::Skybox, transform::Transform};

pub struct Scene {
    pub entities: Vec<(Entity, Transform)>,
    pub lights: Vec<PointLight>,
    pub skybox: Skybox,
}
