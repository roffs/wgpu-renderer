#[repr(C)]
pub struct Vertex {
    position: (f32, f32),
    uv: (f32, f32),
}

impl Vertex {
    pub fn new(position: (f32, f32), uv: (f32, f32)) -> Vertex {
        Vertex { position, uv }
    }
}
