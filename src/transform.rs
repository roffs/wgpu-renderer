use cgmath::{InnerSpace, Matrix, Matrix3, Matrix4, Quaternion, Zero};

#[derive(Debug)]
pub struct Transform {
    pub translation: (f32, f32, f32),
    pub rotation: Quaternion<f32>,
    pub scale: (f32, f32, f32),
}

impl Transform {
    pub fn new(
        translation: (f32, f32, f32),
        rotation: Quaternion<f32>,
        scale: (f32, f32, f32),
    ) -> Transform {
        Transform {
            translation,
            rotation,
            scale,
        }
    }

    pub fn _from_matrix(matrix: Matrix4<f32>) -> Transform {
        let transposed = matrix.transpose();

        let first = transposed.row(0);
        let second = transposed.row(1);
        let third = transposed.row(2);
        let fourth = transposed.row(3);

        let translation = fourth.truncate().into();

        let sx = first.truncate().magnitude();
        let sy = second.truncate().magnitude();
        let sz = third.truncate().magnitude();

        let scale = (sx, sy, sz);

        let rotation = Quaternion::from(Matrix3::from_cols(
            first.truncate() / sx,
            second.truncate() / sy,
            third.truncate() / sz,
        ));

        Transform {
            translation,
            rotation,
            scale,
        }
    }

    pub fn zero() -> Transform {
        Transform {
            translation: (0.0, 0.0, 0.0),
            rotation: Quaternion::<f32>::zero(),
            scale: (1.0, 1.0, 1.0),
        }
    }

    pub fn model(&self) -> Matrix4<f32> {
        let local_translation = Matrix4::from_translation(self.translation.into());
        let local_rotation = Matrix4::from(self.rotation);
        let local_scale = Matrix4::from_nonuniform_scale(self.scale.0, self.scale.1, self.scale.2);

        local_translation * local_rotation * local_scale
    }
}
