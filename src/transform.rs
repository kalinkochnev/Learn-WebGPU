use crate::linalg::SMatrix;

pub trait Transform {
    fn rotation(rad: f32) -> Self;
}

impl Transform for SMatrix<f32, 3, 3> {
    fn rotation(rad: f32) -> SMatrix<f32, 3, 3> {
        SMatrix::new([
            [rad.cos(), -rad.sin(), 0.0],
            [rad.sin(), rad.cos(), 0.0],
            [0.0, 0.0, 0.0]
        ])
    }
}
