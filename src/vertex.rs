use wgpu::{VertexAttribute, VertexBufferLayout};

#[repr(C)]
pub struct Vertex {
    position: (f32, f32, f32),
    uv: (f32, f32),
}

impl<'a> Vertex {
    pub fn new(position: (f32, f32, f32), uv: (f32, f32)) -> Vertex {
        Vertex { position, uv }
    }

    pub fn desc() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: (std::mem::size_of::<f32>() * 3) as u64,
                    shader_location: 1,
                },
            ],
        }
    }
}
