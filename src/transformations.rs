use crate::{matrices::Matrix, tuples::Tuple};

pub struct Transformation {}

impl Transformation {
    pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
        let mut matrix = Matrix::identity(4);

        matrix.set(0, 3, x);
        matrix.set(1, 3, y);
        matrix.set(2, 3, z);

        matrix
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
        let mut matrix = Matrix::identity(4);

        matrix.set(0, 0, x);
        matrix.set(1, 1, y);
        matrix.set(2, 2, z);

        matrix
    }

    pub fn rotation_x(rad: f64) -> Matrix {
        let mut matrix = Matrix::identity(4);

        matrix.set(1, 1, rad.cos());
        matrix.set(1, 2, -rad.sin());
        matrix.set(2, 1, rad.sin());
        matrix.set(2, 2, rad.cos());

        matrix
    }

    pub fn rotation_y(rad: f64) -> Matrix {
        let mut matrix = Matrix::identity(4);

        matrix.set(0, 0, rad.cos());
        matrix.set(0, 2, rad.sin());
        matrix.set(2, 0, -rad.sin());
        matrix.set(2, 2, rad.cos());

        matrix
    }

    pub fn rotation_z(rad: f64) -> Matrix {
        let mut matrix = Matrix::identity(4);

        matrix.set(0, 0, rad.cos());
        matrix.set(0, 1, -rad.sin());
        matrix.set(1, 0, rad.sin());
        matrix.set(1, 1, rad.cos());

        matrix
    }

    fn shearing(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Matrix {
        let mut matrix = Matrix::identity(4);

        matrix.set(0, 1, x_y);
        matrix.set(0, 2, x_z);

        matrix.set(1, 0, y_x);
        matrix.set(1, 2, y_z);

        matrix.set(2, 0, z_x);
        matrix.set(2, 1, z_y);

        matrix
    }

    pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Matrix {
        let forward = (to - from).normalize();
        let left = forward.cross(&up.normalize());
        let true_up = left.cross(&forward);

        let orientation = Matrix::from_vector(
            vec![
                left.x, left.y, left.z, 0.0, true_up.x, true_up.y, true_up.z, 0.0, -forward.x,
                -forward.y, -forward.z, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
            4,
            4,
        );

        orientation * Transformation::translation(-from.x, -from.y, -from.z)
    }
}

#[cfg(test)]
mod tests {

    use std::f64::consts::PI;

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

    #[test]
    fn scale_a_point() {
        let t = Transformation::scaling(2.0, 3.0, 4.0);
        let p1 = Tuple::new_point(-4.0, 6.0, 8.0);
        let p2 = Tuple::new_point(-8.0, 18.0, 32.0);

        assert!(p2 == t * p1);
    }

    #[test]
    fn scale_a_vector() {
        let t = Transformation::scaling(2.0, 3.0, 4.0);
        let v1 = Tuple::new_vector(-4.0, 6.0, 8.0);
        let v2 = Tuple::new_vector(-8.0, 18.0, 32.0);

        assert!(v2 == t * v1);
    }

    #[test]
    fn scale_a_point_by_inverse() {
        let t = Transformation::scaling(2.0, 3.0, 4.0);
        let v1 = Tuple::new_vector(-4.0, 6.0, 8.0);
        let v2 = Tuple::new_vector(-2.0, 2.0, 2.0);

        assert!(v2 == t.invert() * v1);
    }

    #[test]
    fn reflection_a_point() {
        let t = Transformation::scaling(-1.0, 1.0, 1.0);
        let p1 = Tuple::new_point(2.0, 3.0, 4.0);
        let p2 = Tuple::new_point(-2.0, 3.0, 4.0);

        assert!(p2 == t * p1);
    }

    #[test]
    fn rotate_a_point_around_x() {
        let half_quarter = Transformation::rotation_x(PI / 4.0);
        let full_quarter = Transformation::rotation_x(PI / 2.0);
        let p1 = Tuple::new_point(0.0, 1.0, 0.0);
        let p2 = Tuple::new_point(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0);
        let p3 = Tuple::new_point(0.0, 0.0, 1.0);

        assert!(p2 == half_quarter * p1);
        assert!(p3 == full_quarter * p1);
    }

    #[test]
    fn inverse_rotate_a_point_around_x() {
        let half_quarter = Transformation::rotation_x(PI / 4.0);
        let p1 = Tuple::new_point(0.0, 1.0, 0.0);
        let p2 = Tuple::new_point(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);

        assert!(p2 == half_quarter.invert() * p1);
    }

    #[test]
    fn rotate_a_point_around_y() {
        let half_quarter = Transformation::rotation_y(PI / 4.0);
        let full_quarter = Transformation::rotation_y(PI / 2.0);
        let p1 = Tuple::new_point(0.0, 0.0, 1.0);
        let p2 = Tuple::new_point(f64::sqrt(2.0) / 2.0, 0.0, f64::sqrt(2.0) / 2.0);
        let p3 = Tuple::new_point(1.0, 0.0, 0.0);

        assert!(p2 == half_quarter * p1);
        assert!(p3 == full_quarter * p1);
    }

    #[test]
    fn rotate_a_point_around_z() {
        let half_quarter = Transformation::rotation_z(PI / 4.0);
        let full_quarter = Transformation::rotation_z(PI / 2.0);
        let p1 = Tuple::new_point(0.0, 1.0, 0.0);
        let p2 = Tuple::new_point(-f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0, 0.0);
        let p3 = Tuple::new_point(-1.0, 0.0, 0.0);

        assert!(p2 == half_quarter * p1);
        assert!(p3 == full_quarter * p1);
    }

    #[test]
    fn shearing_moves_x_in_proportion_to_y() {
        let t = Transformation::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p1 = Tuple::new_point(2.0, 3.0, 4.0);
        let p2 = Tuple::new_point(5.0, 3.0, 4.0);

        assert!(p2 == t * p1);
    }

    #[test]
    fn shearing_moves_x_in_proportion_to_z() {
        let t = Transformation::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p1 = Tuple::new_point(2.0, 3.0, 4.0);
        let p2 = Tuple::new_point(6.0, 3.0, 4.0);

        assert!(p2 == t * p1);
    }

    #[test]
    fn shearing_moves_y_in_proportion_to_x() {
        let t = Transformation::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p1 = Tuple::new_point(2.0, 3.0, 4.0);
        let p2 = Tuple::new_point(2.0, 5.0, 4.0);

        assert!(p2 == t * p1);
    }

    #[test]
    fn shearing_moves_y_in_proportion_to_z() {
        let t = Transformation::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p1 = Tuple::new_point(2.0, 3.0, 4.0);
        let p2 = Tuple::new_point(2.0, 7.0, 4.0);

        assert!(p2 == t * p1);
    }

    #[test]
    fn shearing_moves_z_in_proportion_to_x() {
        let t = Transformation::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p1 = Tuple::new_point(2.0, 3.0, 4.0);
        let p2 = Tuple::new_point(2.0, 3.0, 6.0);

        assert!(p2 == t * p1);
    }

    #[test]
    fn shearing_moves_z_in_proportion_to_y() {
        let t = Transformation::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p1 = Tuple::new_point(2.0, 3.0, 4.0);
        let p2 = Tuple::new_point(2.0, 3.0, 7.0);

        assert!(p2 == t * p1);
    }

    #[test]
    fn chaining_transformations() {
        let r = Transformation::rotation_x(PI / 2.0);
        let s = Transformation::scaling(5.0, 5.0, 5.0);
        let t = Transformation::translation(10.0, 5.0, 7.0);

        let p1 = Tuple::new_point(1.0, 0.0, 1.0);

        let p2_expected = Tuple::new_point(1.0, -1.0, 0.0);
        let p3_expected = Tuple::new_point(5.0, -5.0, 0.0);
        let p4_expected = Tuple::new_point(15.0, 0.0, 7.0);

        let p2 = r.clone() * p1;
        assert!(p2 == p2_expected);

        let p3 = s.clone() * p2;
        assert!(p3 == p3_expected);

        let p4 = t.clone() * p3;
        assert!(p4 == p4_expected);

        assert!(p4 == t * s * r * p1);
    }

    #[test]
    fn view_transformation_default_orientation() {
        let from = Tuple::new_point(0.0, 0.0, 0.0);
        let to = Tuple::new_point(0.0, 0.0, -1.0);
        let up = Tuple::new_vector(0.0, 1.0, 0.0);

        let t = Transformation::view_transform(from, to, up);
        assert!(t == Matrix::identity(4));
    }

    #[test]
    fn view_transformation_looking_in_positive_z_direction() {
        let from = Tuple::new_point(0.0, 0.0, 0.0);
        let to = Tuple::new_point(0.0, 0.0, 1.0);
        let up = Tuple::new_vector(0.0, 1.0, 0.0);

        let t = Transformation::view_transform(from, to, up);
        assert!(t == Transformation::scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn view_transformation_moves_the_world() {
        let from = Tuple::new_point(0.0, 0.0, 8.0);
        let to = Tuple::new_point(0.0, 0.0, 0.0);
        let up = Tuple::new_vector(0.0, 1.0, 0.0);

        let t = Transformation::view_transform(from, to, up);
        assert!(t == Transformation::translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn arbitrary_view_transformation() {
        let from = Tuple::new_point(1.0, 3.0, 2.0);
        let to = Tuple::new_point(4.0, -2.0, 8.0);
        let up = Tuple::new_vector(1.0, 1.0, 0.0);

        let m = Matrix::from_vector(
            vec![
                -0.5070925528371099,
                0.5070925528371099,
                0.6761234037828132,
                -2.366431913239846,
                0.7677159338596801,
                0.6060915267313263,
                0.12121830534626524,
                -2.8284271247461894,
                -0.35856858280031806,
                0.5976143046671968,
                -0.7171371656006361,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
            ],
            4,
            4,
        );

        let t = Transformation::view_transform(from, to, up);

        assert!(t == m);
    }
}
