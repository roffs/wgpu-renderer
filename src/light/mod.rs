pub struct PointLight {
    position: (f32, f32, f32),
    color: (f32, f32, f32),
}

impl PointLight {
    pub fn new(position: (f32, f32, f32), color: (f32, f32, f32)) -> PointLight {
        PointLight { position, color }
    }

    pub fn to_raw(&self) -> PointLightRaw {
        PointLightRaw {
            position: self.position,
            _padding1: 0.0,
            color: self.color,
            _padding2: 0.0,
        }
    }
}

#[repr(C)]
pub struct PointLightRaw {
    position: (f32, f32, f32),
    _padding1: f32,
    color: (f32, f32, f32),
    _padding2: f32,
}
