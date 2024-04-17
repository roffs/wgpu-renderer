use crate::{light::PointLight, model::Model, skybox::Skybox, transform::Transform};

pub struct Scene<'a> {
    pub entities: Vec<(Model, Transform)>,
    pub lights: Vec<PointLight>,
    pub skybox: Skybox<'a>,
}
