use crate::{entity::Entity, environment_map::EnvironmentMap, light::PointLight};

pub struct Scene {
    pub entities: Vec<Entity>,
    pub lights: Vec<PointLight>,
    pub env_map: EnvironmentMap,
}
