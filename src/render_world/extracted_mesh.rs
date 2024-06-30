use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, Device, IndexFormat, RenderPass,
};

use crate::entity::{Geometry, Vertex};

pub struct ExtractedMesh {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub indices_len: usize,
}

impl ExtractedMesh {
    pub fn new(device: &Device, geometry: &Geometry) -> ExtractedMesh {
        let vertices_data: &[Vertex] = &geometry.vertices;
        let vertex_buffer_data = as_u8_slice(vertices_data);

        let indices_data: &[u16] = &geometry.indices;
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

        ExtractedMesh {
            vertex_buffer,
            index_buffer,
            indices_len: geometry.indices.len(),
        }
    }
}

fn as_u8_slice<T: Sized>(data: &[T]) -> &[u8] {
    let size = std::mem::size_of_val(data);
    unsafe { std::slice::from_raw_parts(data as *const [T] as *const u8, size) }
}

pub trait DrawMesh<'a> {
    fn draw_mesh(&mut self, object: &'a ExtractedMesh);
}

impl<'a> DrawMesh<'a> for RenderPass<'a> {
    fn draw_mesh(&mut self, mesh: &'a ExtractedMesh) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), IndexFormat::Uint16);

        self.draw_indexed(0..mesh.indices_len as u32, 0, 0..1);
    }
}
