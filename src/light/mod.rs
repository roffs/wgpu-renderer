#[repr(C)]
pub struct PointLight {
    position: (f32, f32, f32),
    _padding1: f32,
    color: (f32, f32, f32),
    _padding2: f32,
}

impl PointLight {
    pub fn new(position: (f32, f32, f32), color: (f32, f32, f32)) -> PointLight {
        PointLight {
            position,
            _padding1: 0.0,
            color,
            _padding2: 0.0,
        }
    }
}
