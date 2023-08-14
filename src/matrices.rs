use std::ops;

use crate::tuples::Tuple;
use float_cmp::{ApproxEq, F32Margin};

#[derive(Clone, Debug)]
struct Matrix {
    width: usize,
    height: usize,
    grid: Vec<Vec<f32>>,
}

impl Matrix {
    fn new(width: usize, height: usize) -> Matrix {
        Matrix {
            grid: vec![vec![0.0; width]; height],
            width,
            height,
        }
    }

    fn identity(size: usize) -> Matrix {
        let mut grid = vec![vec![0.0; size]; size];

        for i in 0..size {
            grid[i][i] = 1.0;
        }

        Matrix {
            width: size,
            height: size,
            grid: grid,
        }
    }

    fn from_vector(a: Vec<f32>, width: usize, height: usize) -> Matrix {
        let mut grid = vec![vec![0.0; width]; height];

        for row in 0..height {
            for col in 0..width {
                let value = a.get(width * row + col).unwrap();
                grid[row][col] = *value;
            }
        }

        Matrix {
            grid,
            width,
            height,
        }
    }

    fn get(&self, row: usize, col: usize) -> f32 {
        self.grid[row][col]
    }

    fn set(&mut self, row: usize, col: usize, value: f32) {
        self.grid[row][col] = value
    }

    fn transpose(&self) -> Matrix {
        let mut output = Matrix::new(self.width, self.height);

        for row in 0..self.width {
            for col in 0..self.height {
                output.set(row, col, self.get(col, row))
            }
        }

        output
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        let margin = F32Margin::default();

        if !(self.width == other.width && self.height == other.height) {
            return false;
        }

        for row in 0..self.height {
            for col in 0..self.width {
                if !self.get(row, col).approx_eq(other.get(row, col), margin) {
                    return false;
                }
            }
        }

        true
    }
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Self;

    // We are only interested in 4x4 matrix multiplications, so we can simplify this
    // implementation. No need to be generic.
    fn mul(self, rhs: Matrix) -> Matrix {
        let mut output = Matrix::new(4, 4);

        for row in 0..4 {
            for col in 0..4 {
                let value = self.get(row, 0) * rhs.get(0, col)
                    + self.get(row, 1) * rhs.get(1, col)
                    + self.get(row, 2) * rhs.get(2, col)
                    + self.get(row, 3) * rhs.get(3, col);

                output.set(row, col, value);
            }
        }

        output
    }
}

impl ops::Mul<Tuple> for Matrix {
    type Output = Tuple;

    // We are only interested in 4x4 matrix multiplications, so we can simplify this
    // implementation. No need to be generic.
    fn mul(self, rhs: Tuple) -> Tuple {
        let mut output: Tuple = Tuple::new(0.0, 0.0, 0.0, 0.0);

        for row in 0..4 {
            let value = self.get(row, 0) * rhs.x
                + self.get(row, 1) * rhs.y
                + self.get(row, 2) * rhs.z
                + self.get(row, 3) * rhs.w;

            output.set(row, value);
        }

        output
    }
}

#[cfg(test)]
mod tests {

    use float_cmp::{ApproxEq, F32Margin};

    use super::*;

    #[test]
    fn four_by_four_matrix_is_representable() {
        let matrix = Matrix::from_vector(
            vec![
                1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5,
                16.5,
            ],
            4,
            4,
        );

        assert!(matrix.get(0, 0).approx_eq(1.0, F32Margin::default()));
        assert!(matrix.get(0, 3).approx_eq(4.0, F32Margin::default()));
        assert!(matrix.get(1, 0).approx_eq(5.5, F32Margin::default()));
        assert!(matrix.get(1, 2).approx_eq(7.5, F32Margin::default()));
        assert!(matrix.get(2, 2).approx_eq(11.0, F32Margin::default()));
        assert!(matrix.get(3, 0).approx_eq(13.5, F32Margin::default()));
        assert!(matrix.get(3, 2).approx_eq(15.5, F32Margin::default()));
    }

    #[test]
    fn three_by_three_matrix_is_representable() {
        let matrix =
            Matrix::from_vector(vec![-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0], 3, 3);

        assert!(matrix.get(0, 0).approx_eq(-3.0, F32Margin::default()));
        assert!(matrix.get(1, 1).approx_eq(-2.0, F32Margin::default()));
        assert!(matrix.get(2, 2).approx_eq(1.0, F32Margin::default()));
    }

    #[test]
    fn two_by_two_matrix_is_representable() {
        let matrix = Matrix::from_vector(vec![-3.0, 5.0, 1.0, -2.0], 2, 2);

        assert!(matrix.get(0, 0).approx_eq(-3.0, F32Margin::default()));
        assert!(matrix.get(0, 1).approx_eq(5.0, F32Margin::default()));
        assert!(matrix.get(1, 0).approx_eq(1.0, F32Margin::default()));
        assert!(matrix.get(1, 1).approx_eq(-2.0, F32Margin::default()));
    }

    #[test]
    fn equal_matrices() {
        let a = Matrix::from_vector(vec![-3.0, 0.15 + 0.15 + 0.15, 1.0, -2.0], 2, 2);
        let b = Matrix::from_vector(vec![-3.0, 0.1 + 0.1 + 0.25, 1.0, -2.0], 2, 2);

        assert!(a == b);
    }

    #[test]
    fn not_equal_matrices() {
        let a = Matrix::from_vector(vec![-3.0, 5.0, 1.0, -2.0], 2, 2);
        let b = Matrix::from_vector(vec![3.0, 5.0, 1.0, -2.0], 2, 2);

        assert!(a != b);
    }

    #[test]
    fn matrices_multiplication() {
        let a = Matrix::from_vector(
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ],
            4,
            4,
        );

        let b = Matrix::from_vector(
            vec![
                -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
            ],
            4,
            4,
        );

        let c = Matrix::from_vector(
            vec![
                20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0,
                26.0, 46.0, 42.0,
            ],
            4,
            4,
        );

        assert!(a * b == c)
    }

    #[test]
    fn matrix_tuple_multiplication() {
        let a = Matrix::from_vector(
            vec![
                1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
            ],
            4,
            4,
        );

        let b = Tuple::new(1.0, 2.0, 3.0, 1.0);
        let c = Tuple::new(18.0, 24.0, 33.0, 1.0);

        assert!(a * b == c);
    }

    #[test]
    fn matrix_identity_multiplication() {
        let a = Matrix::from_vector(
            vec![
                0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0, 8.0, 16.0, 32.0,
            ],
            4,
            4,
        );

        let b = Matrix::identity(4);

        assert!(a.clone() * b.clone() == a.clone());
        assert!(b * a.clone() == a.clone())
    }

    #[test]
    fn matrix_transpose() {
        let a = Matrix::from_vector(
            vec![
                0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
            ],
            4,
            4,
        );

        let b = Matrix::from_vector(
            vec![
                0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0,
            ],
            4,
            4,
        );

        assert!(a.transpose() == b)
    }

    #[test]
    fn identity_matrix_transponse() {
        assert!(Matrix::identity(4).transpose() == Matrix::identity(4));
    }
}
