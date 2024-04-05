use wgpu::{Buffer, IndexFormat, RenderPass};

use crate::material::Material;

pub struct Mesh {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub indices_len: u32,
}

impl Mesh {
    pub fn new(vertex_buffer: Buffer, index_buffer: Buffer, indices_len: u32) -> Mesh {
        Mesh {
            vertex_buffer,
            index_buffer,
            indices_len,
        }
    }
}

pub trait DrawMesh<'a> {
    fn draw_mesh(&mut self, mesh: &'a Mesh, material: &'a Material);
}

impl<'a> DrawMesh<'a> for RenderPass<'a> {
    fn draw_mesh(&mut self, mesh: &'a Mesh, material: &'a Material) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), IndexFormat::Uint16);
        self.set_bind_group(2, material.diffuse.bind_group.as_ref().unwrap(), &[]);
        self.draw_indexed(0..mesh.indices_len, 0, 0..1);
    }
}
