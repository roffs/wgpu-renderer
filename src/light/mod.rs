pub struct PointLight {
    pub position: (f32, f32, f32),
    pub color: (f32, f32, f32),
}

impl PointLight {
    pub fn new(position: (f32, f32, f32), color: (f32, f32, f32)) -> PointLight {
        PointLight { position, color }
    }
}
