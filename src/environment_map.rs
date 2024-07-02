use std::ops::Deref;

use crate::texture::CubeMap;

pub struct EnvironmentMap(CubeMap);

impl From<CubeMap> for EnvironmentMap {
    fn from(value: CubeMap) -> Self {
        EnvironmentMap(value)
    }
}

impl Deref for EnvironmentMap {
    type Target = CubeMap;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
