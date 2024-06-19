use cgmath::Vector3;
use wgpu::{VertexAttribute, VertexBufferLayout};

#[repr(C)]
pub struct Vertex {
    position: [f32; 3],
    uv: [f32; 2],
    normal: [f32; 3],
    tangent: [f32; 3],
    bitangent: [f32; 3],
}

impl<'a> Vertex {
    pub fn new(position: [f32; 3], uv: [f32; 2], normal: [f32; 3], tangent: [f32; 3]) -> Vertex {
        let bitangent = Vector3::from(normal).cross(Vector3::from(tangent));

        Vertex {
            position,
            uv,
            normal,
            tangent,
            bitangent: bitangent.into(),
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
                    format: wgpu::VertexFormat::Float32x3,
                    offset: (std::mem::size_of::<f32>() * 8) as u64,
                    shader_location: 3,
                },
                VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: (std::mem::size_of::<f32>() * 11) as u64,
                    shader_location: 4,
                },
            ],
        }
    }
}
