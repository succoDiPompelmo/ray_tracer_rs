use float_cmp::{ApproxEq, F32Margin};

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

    fn from_vector(a: Vec<f32>, width: usize, height: usize) -> Matrix {
        let mut grid = vec![vec![0.0; width]; height];

        for x in 0..width {
            for y in 0..height {
                let value = a.get(width * x + y).unwrap();
                grid[y][x] = *value;
            }
        }

        Matrix {
            grid,
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> f32 {
        self.grid[y][x]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        let margin = F32Margin::default();

        if !(self.width == other.width && self.height == other.height) {
            return false;
        }

        for x in 0..self.width {
            for y in 0..self.height {
                if !self.get(x, y).approx_eq(other.get(x, y), margin) {
                    return false;
                }
            }
        }

        true
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
}
