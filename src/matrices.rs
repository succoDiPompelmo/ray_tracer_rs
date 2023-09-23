use std::ops;

use crate::tuples::Tuple;
use float_cmp::{ApproxEq, F64Margin};

#[derive(Clone, Debug)]
pub struct Matrix {
    width: usize,
    height: usize,
    grid: Vec<Vec<f64>>,
}

impl Matrix {
    fn new(width: usize, height: usize) -> Matrix {
        Matrix {
            grid: vec![vec![0.0; width]; height],
            width,
            height,
        }
    }

    pub fn identity(size: usize) -> Matrix {
        let grid: Vec<Vec<f64>> = (0..size)
            .map(|i| (0..size).map(|j| if i == j { 1.0 } else { 0.0 }).collect())
            .collect();

        Matrix {
            width: size,
            height: size,
            grid,
        }
    }

    pub fn from_vector(a: Vec<f64>, width: usize, height: usize) -> Matrix {
        let grid: Vec<Vec<f64>> = (0..height)
            .map(|row| {
                (0..width)
                    .map(|col| *a.get(width * row + col).unwrap())
                    .collect()
            })
            .collect();

        Matrix {
            grid,
            width,
            height,
        }
    }

    fn get(&self, row: usize, col: usize) -> f64 {
        self.grid[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.grid[row][col] = value
    }

    pub fn transpose(&self) -> Matrix {
        let mut output = Matrix::new(self.width, self.height);

        for row in 0..self.width {
            for col in 0..self.height {
                output.set(row, col, self.get(col, row))
            }
        }

        output
    }

    fn determinant(&self) -> f64 {
        match (self.width, self.height) {
            (x, y) if x != y => panic!("Determinant is a property of square matrices"),
            (2, 2) => self.get(0, 0) * self.get(1, 1) - self.get(0, 1) * self.get(1, 0),
            _ => {
                let mut det = 0.0;

                for col in 0..self.width {
                    det += self.get(0, col) * self.cofactor(0, col);
                }

                det
            }
        }
    }

    fn submatrix(&self, remove_row: usize, remove_col: usize) -> Matrix {
        let mut flat_matrix = vec![];

        for row in 0..self.height {
            for col in 0..self.width {
                if row != remove_row && col != remove_col {
                    flat_matrix.push(self.get(row, col));
                }
            }
        }

        Matrix::from_vector(flat_matrix, self.width - 1, self.height - 1)
    }

    fn minor(&self, target_row: usize, target_col: usize) -> f64 {
        self.submatrix(target_row, target_col).determinant()
    }

    fn cofactor(&self, target_row: usize, target_col: usize) -> f64 {
        match (target_col + target_row) % 2 {
            0 => self.minor(target_row, target_col),
            1 => -self.minor(target_row, target_col),
            _ => panic!("Odd or even"),
        }
    }

    fn is_invertible(&self) -> bool {
        !self.determinant().approx_eq(0.0, F64Margin::default())
    }

    pub fn invert(&self) -> Matrix {
        if !self.is_invertible() {
            panic!("Matrix {:?} cannot be inverted", self)
        }

        let mut inverted = Matrix::new(self.width, self.height);
        let determinant = self.determinant();

        for row in 0..self.height {
            for col in 0..self.width {
                inverted.set(col, row, self.cofactor(row, col) / determinant);
            }
        }

        inverted
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        let margin = F64Margin {
            ulps: 2,
            epsilon: 1e-14,
        };

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

impl ops::Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    // We are only interested in 4x4 matrix multiplications, so we can simplify this
    // implementation. No need to be generic.
    fn mul(self, rhs: &Matrix) -> Matrix {
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

impl ops::Mul<&Tuple> for &Matrix {
    type Output = Tuple;

    // We are only interested in 4x4 matrix multiplications, so we can simplify this
    // implementation. No need to be generic.
    fn mul(self, rhs: &Tuple) -> Tuple {
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

        assert!(matrix.get(0, 0).approx_eq(1.0, F64Margin::default()));
        assert!(matrix.get(0, 3).approx_eq(4.0, F64Margin::default()));
        assert!(matrix.get(1, 0).approx_eq(5.5, F64Margin::default()));
        assert!(matrix.get(1, 2).approx_eq(7.5, F64Margin::default()));
        assert!(matrix.get(2, 2).approx_eq(11.0, F64Margin::default()));
        assert!(matrix.get(3, 0).approx_eq(13.5, F64Margin::default()));
        assert!(matrix.get(3, 2).approx_eq(15.5, F64Margin::default()));
    }

    #[test]
    fn three_by_three_matrix_is_representable() {
        let matrix =
            Matrix::from_vector(vec![-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0], 3, 3);

        assert!(matrix.get(0, 0).approx_eq(-3.0, F64Margin::default()));
        assert!(matrix.get(1, 1).approx_eq(-2.0, F64Margin::default()));
        assert!(matrix.get(2, 2).approx_eq(1.0, F64Margin::default()));
    }

    #[test]
    fn two_by_two_matrix_is_representable() {
        let matrix = Matrix::from_vector(vec![-3.0, 5.0, 1.0, -2.0], 2, 2);

        assert!(matrix.get(0, 0).approx_eq(-3.0, F64Margin::default()));
        assert!(matrix.get(0, 1).approx_eq(5.0, F64Margin::default()));
        assert!(matrix.get(1, 0).approx_eq(1.0, F64Margin::default()));
        assert!(matrix.get(1, 1).approx_eq(-2.0, F64Margin::default()));
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

        assert!(&a * &b == c);
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

        assert!(&a * &b == a);
        assert!(&b * &a == a)
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

    #[test]
    fn two_by_two_matrix_determinant() {
        let matrix = Matrix::from_vector(vec![1.0, 5.0, -3.0, 2.0], 2, 2);

        assert!(matrix.determinant() == 17.0);
    }

    #[test]
    fn submatrix_of_a_three_by_three_matrix() {
        let matrix = Matrix::from_vector(vec![1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0], 3, 3);

        let submatrix = Matrix::from_vector(vec![-3.0, 2.0, 0.0, 6.0], 2, 2);

        assert!(matrix.submatrix(0, 2) == submatrix)
    }

    #[test]
    fn submatrix_of_a_four_by_four_matrix() {
        let matrix = Matrix::from_vector(
            vec![
                -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
            ],
            4,
            4,
        );

        let submatrix =
            Matrix::from_vector(vec![-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0], 3, 3);

        assert!(matrix.submatrix(2, 1) == submatrix);
    }

    #[test]
    fn three_by_three_matrix_minor() {
        let matrix =
            Matrix::from_vector(vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0], 3, 3);

        assert!(matrix.minor(1, 0) == 25.0)
    }

    #[test]
    fn matrix_cofactor() {
        let matrix =
            Matrix::from_vector(vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0], 3, 3);

        assert!(matrix.cofactor(0, 0) == -12.0);
        assert!(matrix.cofactor(1, 0) == -25.0);
    }

    #[test]
    fn three_by_three_matrix_determinant() {
        let matrix = Matrix::from_vector(vec![1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0], 3, 3);

        assert!(matrix.cofactor(0, 0) == 56.0);
        assert!(matrix.cofactor(0, 1) == 12.0);
        assert!(matrix.cofactor(0, 2) == -46.0);

        assert!(matrix.determinant() == -196.0);
    }

    #[test]
    fn four_by_four_matrix_determinant() {
        let matrix = Matrix::from_vector(
            vec![
                -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0,
                -9.0,
            ],
            4,
            4,
        );

        assert!(matrix.cofactor(0, 0) == 690.0);
        assert!(matrix.cofactor(0, 1) == 447.0);
        assert!(matrix.cofactor(0, 2) == 210.0);
        assert!(matrix.cofactor(0, 3) == 51.0);

        assert!(matrix.determinant() == -4071.0);
    }

    #[test]
    fn invertible_matrix() {
        let matrix = Matrix::from_vector(
            vec![
                6.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 6.0, 4.0, -9.0, 3.0, -7.0, 9.0, 1.0, 7.0, -6.0,
            ],
            4,
            4,
        );

        assert!(matrix.is_invertible());
    }

    #[test]
    fn non_invertible_matrix() {
        let matrix = Matrix::from_vector(
            vec![
                -4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0,
            ],
            4,
            4,
        );

        assert!(!matrix.is_invertible());
    }

    #[test]
    fn first_inverse_of_matrix() {
        let matrix = Matrix::from_vector(
            vec![
                -5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0,
                4.0,
            ],
            4,
            4,
        );

        let inverse = Matrix::from_vector(
            vec![
                29.0 / 133.0,
                60.0 / 133.0,
                32.0 / 133.0,
                -6.0 / 133.0,
                -215.0 / 266.0,
                -775.0 / 532.0,
                -59.0 / 133.0,
                277.0 / 532.0,
                -3.0 / 38.0,
                -17.0 / 76.0,
                -1.0 / 19.0,
                15.0 / 76.0,
                -139.0 / 266.0,
                -433.0 / 532.0,
                -40.0 / 133.0,
                163.0 / 532.0,
            ],
            4,
            4,
        );

        assert!(matrix.cofactor(2, 3) == -160.0);
        assert!(matrix.cofactor(3, 2) == 105.0);
        assert!(matrix.invert() == inverse);
    }

    #[test]
    fn second_inverse_of_matrix() {
        let matrix = Matrix::from_vector(
            vec![
                8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, -6.0, 0.0, 9.0, 6.0, -3.0, 0.0, -9.0, -4.0,
            ],
            4,
            4,
        );

        let inverse = Matrix::from_vector(
            vec![
                -2.0 / 13.0,
                -2.0 / 13.0,
                -11.0 / 39.0,
                -7.0 / 13.0,
                -1.0 / 13.0,
                8.0 / 65.0,
                1.0 / 39.0,
                2.0 / 65.0,
                14.0 / 39.0,
                14.0 / 39.0,
                17.0 / 39.0,
                12.0 / 13.0,
                -9.0 / 13.0,
                -9.0 / 13.0,
                -10.0 / 13.0,
                -25.0 / 13.0,
            ],
            4,
            4,
        );

        assert!(matrix.invert() == inverse);
    }

    #[test]
    fn third_inverse_of_matrix() {
        let matrix = Matrix::from_vector(
            vec![
                9.0, 3.0, 0.0, 9.0, -5.0, -2.0, -6.0, -3.0, -4.0, 9.0, 6.0, 4.0, -7.0, 6.0, 6.0,
                2.0,
            ],
            4,
            4,
        );

        let inverse = Matrix::from_vector(
            vec![
                -11.0 / 270.0,
                -7.0 / 90.0,
                13.0 / 90.0,
                -2.0 / 9.0,
                -7.0 / 90.0,
                1.0 / 30.0,
                11.0 / 30.0,
                -1.0 / 3.0,
                -47.0 / 1620.0,
                -79.0 / 540.0,
                -59.0 / 540.0,
                7.0 / 54.0,
                8.0 / 45.0,
                1.0 / 15.0,
                -4.0 / 15.0,
                1.0 / 3.0,
            ],
            4,
            4,
        );

        assert!(matrix.invert() == inverse);
    }

    #[test]
    fn multiply_matrix_by_inverse() {
        let a = Matrix::from_vector(
            vec![
                3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0,
                1.0,
            ],
            4,
            4,
        );

        let b = Matrix::from_vector(
            vec![
                8.0, 2.0, 2.0, 2.0, 3.0, -1.0, 7.0, 0.0, 7.0, 0.0, 5.0, 4.0, 6.0, -2.0, 0.0, 5.0,
            ],
            4,
            4,
        );

        let c = &a * &b;
        assert!(a == &c * &b.invert());
    }

    #[test]
    fn inverse_of_identity_matrix() {
        let identity = Matrix::identity(4);

        assert!(identity == identity.invert())
    }

    #[test]
    fn multiply_matrix_by_own_inverse() {
        let a = Matrix::from_vector(
            vec![
                3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0,
                1.0,
            ],
            4,
            4,
        );

        assert!(Matrix::identity(4) == &a * &a.invert())
    }
}
