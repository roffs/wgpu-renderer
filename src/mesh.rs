use wgpu::{Buffer, BufferDescriptor, BufferUsages, Device, Queue};

use crate::vertex::Vertex;

pub struct Mesh {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub indices_len: u32,
}

impl Mesh {
    pub fn new(device: &Device, queue: &Queue, vertices: &[Vertex], indices: &[u16]) -> Mesh {
        let vertex_buffer_data = as_u8_slice(vertices);
        let index_buffer_data = as_u8_slice(indices);

        let vertex_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Vertex buffer"),
            size: vertex_buffer_data.len() as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        queue.write_buffer(&vertex_buffer, 0, vertex_buffer_data);

        let index_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Index buffer"),
            size: index_buffer_data.len() as u64,
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        queue.write_buffer(&index_buffer, 0, index_buffer_data);

        Mesh {
            vertex_buffer,
            index_buffer,
            indices_len: indices.len() as u32,
        }
    }

    pub fn cube(device: &Device, queue: &Queue) -> Mesh {
        let vertices = &[
            // FRONT
            Vertex::new((-0.5, -0.5, 0.5), (0.0, 0.0)),
            Vertex::new((0.5, -0.5, 0.5), (1.0, 0.0)),
            Vertex::new((0.5, 0.5, 0.5), (1.0, 1.0)),
            Vertex::new((-0.5, 0.5, 0.5), (0.0, 1.0)),
            // BACK
            Vertex::new((-0.5, -0.5, -0.5), (1.0, 0.0)),
            Vertex::new((0.5, -0.5, -0.5), (0.0, 0.0)),
            Vertex::new((0.5, 0.5, -0.5), (0.0, 1.0)),
            Vertex::new((-0.5, 0.5, -0.5), (1.0, 1.0)),
            // LEFT
            Vertex::new((-0.5, -0.5, -0.5), (0.0, 0.0)),
            Vertex::new((-0.5, -0.5, 0.5), (1.0, 0.0)),
            Vertex::new((-0.5, 0.5, 0.5), (1.0, 1.0)),
            Vertex::new((-0.5, 0.5, -0.5), (0.0, 1.0)),
            // RIGHT
            Vertex::new((0.5, -0.5, -0.5), (1.0, 0.0)),
            Vertex::new((0.5, -0.5, 0.5), (0.0, 0.0)),
            Vertex::new((0.5, 0.5, 0.5), (0.0, 1.0)),
            Vertex::new((0.5, 0.5, -0.5), (1.0, 1.0)),
            // TOP
            Vertex::new((-0.5, 0.5, 0.5), (0.0, 0.0)),
            Vertex::new((0.5, 0.5, 0.5), (1.0, 0.0)),
            Vertex::new((0.5, 0.5, -0.5), (1.0, 1.0)),
            Vertex::new((-0.5, 0.5, -0.5), (0.0, 1.0)),
            // BOTTOM
            Vertex::new((-0.5, -0.5, 0.5), (1.0, 0.0)),
            Vertex::new((0.5, -0.5, 0.5), (0.0, 0.0)),
            Vertex::new((0.5, -0.5, -0.5), (0.0, 1.0)),
            Vertex::new((-0.5, -0.5, -0.5), (1.0, 1.0)),
        ];

        let indices: &[u16] = &[
            0, 1, 2, 0, 2, 3, // FRONT
            4, 6, 5, 4, 7, 6, // BACK
            8, 9, 10, 8, 10, 11, // LEFT
            12, 14, 13, 12, 15, 14, // RIGHT
            16, 17, 18, 16, 18, 19, // TOP
            20, 22, 21, 20, 23, 22, // BOTTOM
        ];

        Mesh::new(device, queue, vertices, indices)
    }
}

fn as_u8_slice<T: Sized>(data: &[T]) -> &[u8] {
    let size = std::mem::size_of_val(data);
    unsafe { std::slice::from_raw_parts(data as *const [T] as *const u8, size) }
}
