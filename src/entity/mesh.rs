use crate::entity::Geometry;

pub struct Mesh {
    pub primitives: Vec<(Geometry, usize)>,
}
