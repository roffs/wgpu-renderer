use wgpu::{Device, Queue};

use crate::model::Vertex;

use super::Mesh;

impl Mesh {
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
