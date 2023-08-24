use crate::matrices::Matrix;

struct Transformation {}

impl Transformation {
    fn translation(x: f64, y: f64, z: f64) -> Matrix {
        let mut matrix = Matrix::identity(4);

        matrix.set(0, 3, x);
        matrix.set(1, 3, y);
        matrix.set(2, 3, z);

        matrix
    }
}

#[cfg(test)]
mod tests {

    use crate::tuples::Tuple;

    use super::*;

    #[test]
    fn multiply_by_tranlation_matrix() {
        let t = Transformation::translation(5.0, -3.0, 2.0);
        let p1 = Tuple::new_point(-3.0, 4.0, 5.0);
        let p2 = Tuple::new_point(2.0, 1.0, 7.0);

        assert!(p2 == t * p1);
    }

    #[test]
    fn multiply_by_inverse_translation_matrix() {
        let t = Transformation::translation(5.0, -3.0, 2.0);
        let p1 = Tuple::new_point(-3.0, 4.0, 5.0);
        let p2 = Tuple::new_point(-8.0, 7.0, 3.0);

        assert!(p2 == t.invert() * p1);
    }

    #[test]
    fn translation_do_not_affect_vectors() {
        let t = Transformation::translation(5.0, -3.0, 2.0);
        let v = Tuple::new_vector(-3.0, 4.0, 5.0);

        assert!(v == t * v);
    }
}