use wgpu::Device;

use crate::model::{Mesh, Vertex};

impl Mesh {
    pub fn plane(device: &Device) -> Mesh {
        let vertices = &[
            Vertex::new((-0.5, 0.0, 0.5), (0.0, 0.0), (0.0, 1.0, 0.0), None, None),
            Vertex::new((0.5, 0.0, 0.5), (1.0, 0.0), (0.0, 1.0, 0.0), None, None),
            Vertex::new((0.5, 0.0, -0.5), (1.0, 1.0), (0.0, 1.0, 0.0), None, None),
            Vertex::new((-0.5, 0.0, -0.5), (0.0, 1.0), (0.0, 1.0, 0.0), None, None),
        ];

        let indices: &[u16] = &[0, 1, 2, 0, 2, 3];

        Mesh::new(device, vertices, indices)
    }
}
