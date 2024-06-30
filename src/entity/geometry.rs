use super::Vertex;

pub struct Geometry {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl Geometry {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u16>) -> Geometry {
        Geometry { vertices, indices }
    }

    pub fn plane() -> Geometry {
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

        Geometry::new(vertices, indices)
    }

    pub fn cube() -> Geometry {
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

        Geometry::new(vertices, indices)
    }
}
