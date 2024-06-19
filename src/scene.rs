use crate::{light::PointLight, resources::Entity, skybox::Skybox, transform::Transform};

pub struct Scene {
    pub entities: Vec<(Entity, Transform)>,
    pub lights: Vec<PointLight>,
    pub skybox: Skybox,
}
