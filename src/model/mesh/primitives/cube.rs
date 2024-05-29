use wgpu::Device;

use crate::model::{Mesh, Vertex};

impl Mesh {
    pub fn cube(device: &Device) -> Mesh {
        let vertices = &[
            // FRONT
            Vertex::new((-0.5, -0.5, 0.5), (0.0, 0.0), (0.0, 0.0, 1.0), None, None),
            Vertex::new((0.5, -0.5, 0.5), (1.0, 0.0), (0.0, 0.0, 1.0), None, None),
            Vertex::new((0.5, 0.5, 0.5), (1.0, 1.0), (0.0, 0.0, 1.0), None, None),
            Vertex::new((-0.5, 0.5, 0.5), (0.0, 1.0), (0.0, 0.0, 1.0), None, None),
            // BACK
            Vertex::new((-0.5, -0.5, -0.5), (1.0, 0.0), (0.0, 0.0, -1.0), None, None),
            Vertex::new((0.5, -0.5, -0.5), (0.0, 0.0), (0.0, 0.0, -1.0), None, None),
            Vertex::new((0.5, 0.5, -0.5), (0.0, 1.0), (0.0, 0.0, -1.0), None, None),
            Vertex::new((-0.5, 0.5, -0.5), (1.0, 1.0), (0.0, 0.0, -1.0), None, None),
            // LEFT
            Vertex::new((-0.5, -0.5, -0.5), (0.0, 0.0), (-1.0, 0.0, 0.0), None, None),
            Vertex::new((-0.5, -0.5, 0.5), (1.0, 0.0), (-1.0, 0.0, 0.0), None, None),
            Vertex::new((-0.5, 0.5, 0.5), (1.0, 1.0), (-1.0, 0.0, 0.0), None, None),
            Vertex::new((-0.5, 0.5, -0.5), (0.0, 1.0), (-1.0, 0.0, 0.0), None, None),
            // RIGHT
            Vertex::new((0.5, -0.5, -0.5), (1.0, 0.0), (1.0, 0.0, 0.0), None, None),
            Vertex::new((0.5, -0.5, 0.5), (0.0, 0.0), (1.0, 0.0, 0.0), None, None),
            Vertex::new((0.5, 0.5, 0.5), (0.0, 1.0), (1.0, 0.0, 0.0), None, None),
            Vertex::new((0.5, 0.5, -0.5), (1.0, 1.0), (1.0, 0.0, 0.0), None, None),
            // TOP
            Vertex::new((-0.5, 0.5, 0.5), (0.0, 0.0), (0.0, 1.0, 0.0), None, None),
            Vertex::new((0.5, 0.5, 0.5), (1.0, 0.0), (0.0, 1.0, 0.0), None, None),
            Vertex::new((0.5, 0.5, -0.5), (1.0, 1.0), (0.0, 1.0, 0.0), None, None),
            Vertex::new((-0.5, 0.5, -0.5), (0.0, 1.0), (0.0, 1.0, 0.0), None, None),
            // BOTTOM
            Vertex::new((-0.5, -0.5, 0.5), (1.0, 0.0), (0.0, -1.0, 0.0), None, None),
            Vertex::new((0.5, -0.5, 0.5), (0.0, 0.0), (0.0, -1.0, 0.0), None, None),
            Vertex::new((0.5, -0.5, -0.5), (0.0, 1.0), (0.0, -1.0, 0.0), None, None),
            Vertex::new((-0.5, -0.5, -0.5), (1.0, 1.0), (0.0, -1.0, 0.0), None, None),
        ];

        let indices: &[u16] = &[
            0, 1, 2, 0, 2, 3, // FRONT
            4, 6, 5, 4, 7, 6, // BACK
            8, 9, 10, 8, 10, 11, // LEFT
            12, 14, 13, 12, 15, 14, // RIGHT
            16, 17, 18, 16, 18, 19, // TOP
            20, 22, 21, 20, 23, 22, // BOTTOM
        ];

        Mesh::new(device, vertices, indices)
    }
}
