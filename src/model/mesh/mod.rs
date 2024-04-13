mod primitives;
mod vertex;

use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, Device, IndexFormat, RenderPass,
};

use crate::material::Material;

pub use vertex::Vertex;

pub struct Mesh {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub indices_len: u32,
}

impl Mesh {
    pub fn new(device: &Device, vertices: &[Vertex], indices: &[u16]) -> Mesh {
        let vertex_buffer_data = as_u8_slice(vertices);
        let index_buffer_data = as_u8_slice(indices);

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

        Mesh {
            vertex_buffer,
            index_buffer,
            indices_len: indices.len() as u32,
        }
    }
}

fn as_u8_slice<T: Sized>(data: &[T]) -> &[u8] {
    let size = std::mem::size_of_val(data);
    unsafe { std::slice::from_raw_parts(data as *const [T] as *const u8, size) }
}

pub trait DrawMesh<'a> {
    fn draw_mesh(&mut self, mesh: &'a Mesh, material: &'a Material);
}

impl<'a> DrawMesh<'a> for RenderPass<'a> {
    fn draw_mesh(&mut self, mesh: &'a Mesh, material: &'a Material) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), IndexFormat::Uint16);
        self.set_bind_group(2, &material.bind_group, &[]);
        self.draw_indexed(0..mesh.indices_len, 0, 0..1);
    }
}
