use wgpu::{Buffer, BufferDescriptor, BufferUsages, Device, Queue};

use crate::vertex::Vertex;

pub struct Mesh {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
}

impl Mesh {
    pub fn new(device: &Device, queue: &Queue) -> Mesh {
        let vertex_buffer_data = &[
            Vertex::new((-0.5, -0.5), (0.0, 0.0)),
            Vertex::new((0.5, -0.5), (1.0, 0.0)),
            Vertex::new((0.5, 0.5), (1.0, 1.0)),
            Vertex::new((-0.5, 0.5), (0.0, 1.0)),
        ];
        let vertex_buffer_data = as_u8_slice(vertex_buffer_data);

        let vertex_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Vertex buffer"),
            size: vertex_buffer_data.len() as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        queue.write_buffer(&vertex_buffer, 0, vertex_buffer_data);

        let index_buffer_data: &[u16] = &[0, 1, 2, 0, 2, 3];
        let index_buffer_data = as_u8_slice(index_buffer_data);

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
        }
    }
}

fn as_u8_slice<T: Sized>(data: &[T]) -> &[u8] {
    let size = std::mem::size_of_val(data);
    unsafe { std::slice::from_raw_parts(data as *const [T] as *const u8, size) }
}