use wgpu::{VertexAttribute, VertexBufferLayout};

#[repr(C)]
#[derive(Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub normal: [f32; 3],
    pub tangent: [f32; 4],
}

impl<'a> Vertex {
    pub fn new(position: [f32; 3], uv: [f32; 2], normal: [f32; 3], tangent: [f32; 4]) -> Vertex {
        Vertex {
            position,
            uv,
            normal,
            tangent,
        }
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
                VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: (std::mem::size_of::<f32>() * 5) as u64,
                    shader_location: 2,
                },
                VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: (std::mem::size_of::<f32>() * 8) as u64,
                    shader_location: 3,
                },
            ],
        }
    }
}
