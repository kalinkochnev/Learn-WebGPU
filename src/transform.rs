use num::{Num, Float};

use crate::linalg::SMatrix;

trait Transform {
    fn rotation(rad: f32) -> Self;
}

impl Transform for SMatrix<f32, 2, 2> {
    fn rotation(rad: f32) -> SMatrix<f32, 2, 2> {
        SMatrix::new([
            [rad.cos(), -rad.sin()],
            [rad.sin(), rad.cos()]
        ])
    }
}
