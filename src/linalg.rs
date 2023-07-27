
use std::ops::{Mul, AddAssign, MulAssign, Add};

use num::Num;

type Vector<T, const Dim: usize> = [T; Dim];
struct Scalar<T>(T);

/// Computes the dot product of two equal slices
pub fn dot_prod<T: Num + Copy>(a: &[T], b: &[T]) -> T {
    assert!(a.len() == b.len(), "Must have two slices of equal size");
    let mut sum = T::zero();
    for (item_a, item_b) in a.iter().zip(b.iter()) {
        sum = sum + (*item_a) * (*item_b);
    }
    sum
}

/// Constant generic parameter (R,C) represent the rows and columns
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct SMatrix<T, const ROWS: usize, const COLS: usize> {
    entries: [[T; COLS]; ROWS]
}
impl<T, const ROWS: usize, const COLS: usize> SMatrix<T, ROWS, COLS> where T: Num + Copy  + Clone{

    pub fn new(entries: [[T; COLS]; ROWS]) -> Self {
        SMatrix { entries }
    }

    pub fn zero() -> Self {
        Self::new([[T::zero(); COLS]; ROWS])
    }

    pub fn shape(&self) -> (usize, usize) {
        return (ROWS, COLS)
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        match self.entries.get(row) {
            Some(row) => return row.get(col),
            None => return None
        }
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        match self.entries.get_mut(row) {
            Some(row) => return row.get_mut(col),
            None => return None
        }

    }

    pub fn col(&self, col: usize) -> Option<Vector<T, ROWS>> {
        let mut column_vec = [T::zero(); ROWS];
        for row_index in 0..ROWS {
            match self.get(row_index, col) {
                Some(entry) => column_vec[row_index] = *entry,
                None => return None
            }
        }
        Some(column_vec)
    }

    pub fn row(&self, row: usize) -> Option<&Vector<T, COLS>> {
        self.entries.get(row)
    }

    pub fn set_col(&mut self, col: usize, vector: &Vector<T, ROWS>) {
        for row in 0..ROWS {
            self.entries[row][col] = vector[row];
        }
    }

    pub fn transpose(&self) -> SMatrix<T, COLS, ROWS> {
        let mut output = SMatrix::zero();
        for (index, row) in self.entries.iter().enumerate() {
            output.set_col(index, row)
        }
        output
    }

}

impl<T, const ROWS: usize> SMatrix<T, ROWS, 1> where T: Num + Copy {
    pub fn vector(v: [T; ROWS]) -> Self {
        let mut values = [[T::zero(); 1]; ROWS];
        for r in 0..ROWS {
            values[r][0] = v[r];
        }
        SMatrix::new(values)
    }
}

impl<T, const ROWS_A: usize, const COLS_A: usize, const COLS_B: usize> Mul<SMatrix<T, COLS_A, COLS_B >>
    for SMatrix<T, ROWS_A, COLS_A>
where T: Num + Copy + AddAssign
{
    /// The output has the same number of rows as A but same number of columns as B
    type Output = SMatrix<T, ROWS_A, COLS_B>;

    /// Apply an MxN matrix (A) onto an NxO matrix (B: self) like AB
    /// The output is an MxO matrix
    fn mul(self, rhs: SMatrix<T, COLS_A, COLS_B>) -> Self::Output {
        let mut result = SMatrix::zero();

        // For each column in B (self)
        for b_col in 0..COLS_B {
            for a_row in 0..ROWS_A {
                for a_col in 0..COLS_A {
                    // Imagine row is fixed, so we compute dot product between
                    // row 0 with col 0, then row1 with col 0, etc
                    let a_entry = self.entries[a_row][a_col];
                    let b_entry = rhs.entries[a_col][b_col];
                    result.entries[a_row][b_col] += a_entry * b_entry;
                }
            }
        }

        result
   }
}

/// Implementing scalar multiplication commutatively
impl<T, const ROWS: usize, const COLS: usize> Mul<Scalar<T>> for SMatrix<T, ROWS, COLS>
    where T: Num + Copy + Clone + MulAssign
{
    type Output = SMatrix<T, ROWS, COLS>;

    fn mul(mut self, rhs: Scalar<T>) -> Self::Output {
        for r in 0..ROWS {
            for c in 0..COLS {
                self.entries[r][c] *= rhs.0;
            }
        }

        self
    }
}

/// Scalar multiplication is commutative
impl<T, const ROWS: usize, const COLS: usize> Mul<SMatrix<T, ROWS, COLS>> for Scalar<T>
    where T: Num + Copy + Clone + MulAssign
{
    type Output = SMatrix<T, ROWS, COLS>;

    fn mul(self, rhs: SMatrix<T, ROWS, COLS>) -> Self::Output {
        rhs * self 
    }
}


/// For matrices of same size, addition is defined
impl<T, const ROWS: usize, const COLS: usize> Add<SMatrix<T, ROWS, COLS>> for SMatrix<T, ROWS, COLS>
    where T: AddAssign + Copy
{
    type Output = SMatrix<T, ROWS, COLS>;

    fn add(mut self, rhs: SMatrix<T, ROWS, COLS>) -> Self::Output {
       for r in 0..ROWS {
           for c in 0..COLS {
                self.entries[r][c] += rhs.entries[r][c];
           }
       } 

       self
    }
}

#[cfg(test)]
mod tests {
    use crate::linalg::{dot_prod, Scalar};

    use super::SMatrix;

    fn basic_matrix() -> SMatrix<i32, 3, 3> {
        SMatrix::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]])
    }

    #[test]
    fn initialization() {
        let expected_entries = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
        assert_eq!(SMatrix::new(expected_entries).entries, expected_entries);

        let zero_mat = SMatrix::<i32, 4, 4>::zero();
        assert_eq!(zero_mat.shape(), (4, 4));
        assert_eq!(zero_mat.entries.into_iter().flatten().collect::<Vec<i32>>(), vec![0; 16]);

    }

    #[test]
    fn dot_product() {
        let a = [0, -1, 2, 10];
        let b = [-5, 4, 3, 2];
        assert_eq!(dot_prod(&a, &b), 22);
    }

    #[test]
    fn get_col_row() {
        let mat = basic_matrix(); assert_eq!(mat.col(0), Some([1, 4, 7]));
        assert_eq!(mat.row(0), Some(&[1, 2, 3]));
        assert_eq!(mat.col(0), Some([1, 4, 7]));
    }


    #[test]
    fn matrix_transpose() {
        let mat = basic_matrix();
        let expected = SMatrix::new([[1, 4, 7], [2, 5, 8], [3, 6, 9]]);
        assert_eq!(mat.transpose().entries, expected.entries);
    }

    #[test]
    fn apply_transformation() {
        let identity = SMatrix::new([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
        let vector = SMatrix::vector([1, 1, 1]);
        assert_eq!(identity * vector, vector);

        let scale_2x = SMatrix::new([[2, 0, 0], [0, 2, 0], [0, 0, 2]]);
        assert_eq!(scale_2x * vector, vector * Scalar(2));
        assert_eq!(scale_2x * vector, Scalar(2) * vector);

        // keep z axis untouched
        let rotate_90 = SMatrix::new([
            [0, -1, 0],
            [1, 0, 0],
            [0, 0, 1 ]
        ]);
        assert_eq!(rotate_90 * vector, SMatrix::vector([-1, 1, 1]));
    }

}
