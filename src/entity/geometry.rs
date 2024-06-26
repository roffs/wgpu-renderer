use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, Device, IndexFormat, RenderPass,
};

use crate::material::Material;

use super::Vertex;

pub struct Geometry {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,

    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
}

impl Geometry {
    pub fn new(device: &Device, vertices: Vec<Vertex>, indices: Vec<u16>) -> Geometry {
        let vertices_data: &[Vertex] = &vertices;
        let vertex_buffer_data = as_u8_slice(vertices_data);

        let indices_data: &[u16] = &indices;
        let index_buffer_data = as_u8_slice(indices_data);

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex buffer"),
            contents: vertex_buffer_data,
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index buffer"),
            contents: index_buffer_data,
            usage: BufferUsages::INDEX,
        });

        Geometry {
            vertices,
            indices,
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn plane(device: &Device) -> Geometry {
        let vertices = vec![
            Vertex::new(
                [-0.5, 0.0, 0.5],
                [0.0, 1.0],
                [0.0, 1.0, 0.0],
                [1.0, 0.0, 0.0],
            ),
            Vertex::new(
                [0.5, 0.0, 0.5],
                [1.0, 1.0],
                [0.0, 1.0, 0.0],
                [1.0, 0.0, 0.0],
            ),
            Vertex::new(
                [0.5, 0.0, -0.5],
                [1.0, 0.0],
                [0.0, 1.0, 0.0],
                [1.0, 0.0, 0.0],
            ),
            Vertex::new(
                [-0.5, 0.0, -0.5],
                [0.0, 0.0],
                [0.0, 1.0, 0.0],
                [1.0, 0.0, 0.0],
            ),
        ];
        let indices = vec![0, 1, 2, 0, 2, 3];

        Geometry::new(device, vertices, indices)
    }

    pub fn cube(device: &Device) -> Geometry {
        let vertices = vec![
            // FRONT
            Vertex::new(
                [-0.5, -0.5, 0.5],
                [0.0, 0.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 0.0],
            ),
            Vertex::new(
                [0.5, -0.5, 0.5],
                [1.0, 0.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 0.0],
            ),
            Vertex::new(
                [0.5, 0.5, 0.5],
                [1.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 0.0],
            ),
            Vertex::new(
                [-0.5, 0.5, 0.5],
                [0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 0.0],
            ),
            // BACK
            Vertex::new(
                [-0.5, -0.5, -0.5],
                [1.0, 0.0],
                [0.0, 0.0, -1.0],
                [0.0, 0.0, 0.0],
            ),
            Vertex::new(
                [0.5, -0.5, -0.5],
                [0.0, 0.0],
                [0.0, 0.0, -1.0],
                [0.0, 0.0, 0.0],
            ),
            Vertex::new(
                [0.5, 0.5, -0.5],
                [0.0, 1.0],
                [0.0, 0.0, -1.0],
                [0.0, 0.0, 0.0],
            ),
            Vertex::new(
                [-0.5, 0.5, -0.5],
                [1.0, 1.0],
                [0.0, 0.0, -1.0],
                [0.0, 0.0, 0.0],
            ),
            // LEFT
            Vertex::new(
                [-0.5, -0.5, -0.5],
                [0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
            ),
            Vertex::new(
                [-0.5, -0.5, 0.5],
                [1.0, 0.0],
                [-1.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
            ),
            Vertex::new(
                [-0.5, 0.5, 0.5],
                [1.0, 1.0],
                [-1.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
            ),
            Vertex::new(
                [-0.5, 0.5, -0.5],
                [0.0, 1.0],
                [-1.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
            ),
            // RIGHT
            Vertex::new(
                [0.5, -0.5, -0.5],
                [1.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
            ),
            Vertex::new(
                [0.5, -0.5, 0.5],
                [0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
            ),
            Vertex::new(
                [0.5, 0.5, 0.5],
                [0.0, 1.0],
                [1.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
            ),
            Vertex::new(
                [0.5, 0.5, -0.5],
                [1.0, 1.0],
                [1.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
            ),
            // TOP
            Vertex::new(
                [-0.5, 0.5, 0.5],
                [0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0],
            ),
            Vertex::new(
                [0.5, 0.5, 0.5],
                [1.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0],
            ),
            Vertex::new(
                [0.5, 0.5, -0.5],
                [1.0, 1.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0],
            ),
            Vertex::new(
                [-0.5, 0.5, -0.5],
                [0.0, 1.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0],
            ),
            // BOTTOM
            Vertex::new(
                [-0.5, -0.5, 0.5],
                [1.0, 0.0],
                [0.0, -1.0, 0.0],
                [0.0, 0.0, 0.0],
            ),
            Vertex::new(
                [0.5, -0.5, 0.5],
                [0.0, 0.0],
                [0.0, -1.0, 0.0],
                [0.0, 0.0, 0.0],
            ),
            Vertex::new(
                [0.5, -0.5, -0.5],
                [0.0, 1.0],
                [0.0, -1.0, 0.0],
                [0.0, 0.0, 0.0],
            ),
            Vertex::new(
                [-0.5, -0.5, -0.5],
                [1.0, 1.0],
                [0.0, -1.0, 0.0],
                [0.0, 0.0, 0.0],
            ),
        ];

        let indices = vec![
            0, 1, 2, 0, 2, 3, // FRONT
            4, 6, 5, 4, 7, 6, // BACK
            8, 9, 10, 8, 10, 11, // LEFT
            12, 14, 13, 12, 15, 14, // RIGHT
            16, 17, 18, 16, 18, 19, // TOP
            20, 22, 21, 20, 23, 22, // BOTTOM
        ];

        Geometry::new(device, vertices, indices)
    }
}

fn as_u8_slice<T: Sized>(data: &[T]) -> &[u8] {
    let size = std::mem::size_of_val(data);
    unsafe { std::slice::from_raw_parts(data as *const [T] as *const u8, size) }
}

pub trait DrawGeometry<'a> {
    fn draw_geometry(&mut self, primitive: &'a Geometry, material: &'a Material);
}

impl<'a> DrawGeometry<'a> for RenderPass<'a> {
    fn draw_geometry(&mut self, primitive: &'a Geometry, material: &'a Material) {
        self.set_vertex_buffer(0, primitive.vertex_buffer.slice(..));
        self.set_index_buffer(primitive.index_buffer.slice(..), IndexFormat::Uint16);

        self.set_bind_group(2, material, &[]);
        self.draw_indexed(0..primitive.indices.len() as u32, 0, 0..1);
    }
}
