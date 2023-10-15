use std::ops;

use float_cmp::{ApproxEq, F64Margin};

#[derive(Clone, Copy, Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }

    pub fn new_point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 1.0 }
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 0.0 }
    }

    pub fn new_color(r: f64, g: f64, b: f64) -> Tuple {
        // w value for colors are useless, so for no particular reason is set to 0
        // to avoid interaction with other components
        Tuple {
            x: r,
            y: g,
            z: b,
            w: 0.0,
        }
    }

    pub fn black() -> Tuple {
        Self::new_color(0.0, 0.0, 0.0)
    }

    pub fn white() -> Tuple {
        Self::new_color(1.0, 1.0, 1.0)
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0) + self.w.powf(2.0)).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        let magnitude = self.magnitude();
        Tuple::new(
            self.x / magnitude,
            self.y / magnitude,
            self.z / magnitude,
            self.w / magnitude,
        )
    }

    pub fn dot(&self, rhs: &Tuple) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn cross(&self, rhs: &Tuple) -> Tuple {
        Tuple::new_vector(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn hadamard_product(&self, rhs: &Tuple) -> Tuple {
        Tuple::new_color(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }

    pub fn set(&mut self, idx: usize, value: f64) {
        match idx {
            0 => self.x = value,
            1 => self.y = value,
            2 => self.z = value,
            3 => self.w = value,
            _ => (),
        }
    }

    pub fn reflect(&self, normal: &Tuple) -> Tuple {
        self - &(normal * 2.0 * self.dot(normal))
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        let margin = F64Margin {
            ulps: 2,
            epsilon: 1e-14,
        };

        self.x.approx_eq(other.x, margin)
            && self.y.approx_eq(other.y, margin)
            && self.z.approx_eq(other.z, margin)
            && self.w == other.w
    }
}

impl ops::Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Tuple) -> Tuple {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl ops::Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Tuple {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl ops::Mul<f64> for &Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl ops::Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Tuple) -> Tuple {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl ops::Sub for &Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl ops::Div<f64> for Tuple {
    type Output = Self;

    fn div(self, rhs: f64) -> Tuple {
        Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl ops::Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    impl Tuple {
        fn is_vector(&self) -> bool {
            self.w == 0.0
        }

        fn is_point(&self) -> bool {
            self.w == 1.0
        }
    }

    #[test]
    fn tuple_is_a_point() {
        let tuple = Tuple::new_point(10.0, 11.0, 12.0);

        assert_eq!(tuple.x, 10.0);
        assert_eq!(tuple.y, 11.0);
        assert_eq!(tuple.z, 12.0);

        assert!(tuple.is_point());
        assert!(!tuple.is_vector());
    }

    #[test]
    fn tuple_is_a_vector() {
        let tuple = Tuple::new_vector(1.0, 2.0, 3.0);

        assert_eq!(tuple.x, 1.0);
        assert_eq!(tuple.y, 2.0);
        assert_eq!(tuple.z, 3.0);

        assert!(!tuple.is_point());
        assert!(tuple.is_vector());
    }

    #[test]
    fn point_to_point_equal() {
        let point_1 = Tuple::new_point(0.15 + 0.15 + 0.15, 2.0, 3.0);
        let point_2 = Tuple::new_point(0.1 + 0.1 + 0.25, 2.0, 3.0);

        assert!(point_1 == point_2)
    }

    #[test]
    fn point_to_point_not_equal() {
        let point_1 = Tuple::new_point(2.0, 2.0, 3.0);
        let point_2 = Tuple::new_point(1.0, 2.0, 3.0);

        assert!(!(point_1 == point_2))
    }

    #[test]
    fn point_to_vector_not_equal() {
        let point_1 = Tuple::new_point(1.0, 2.0, 3.0);
        let point_2 = Tuple::new_vector(1.0, 2.0, 3.0);

        assert!(!(point_1 == point_2))
    }

    #[test]
    fn add_point_to_point() {
        let point_1 = Tuple::new_point(1.0, 2.0, 3.0);
        let point_2 = Tuple::new_point(1.0, 2.0, 3.0);

        let point_3 = Tuple::new(2.0, 4.0, 6.0, 2.0);

        assert!((point_1 + point_2) == point_3)
    }

    #[test]
    fn add_point_to_vector() {
        let point_1 = Tuple::new_point(1.0, 2.0, 3.0);
        let point_2 = Tuple::new_vector(1.0, 2.0, 3.0);

        let point_3 = Tuple::new_point(2.0, 4.0, 6.0);

        assert!((point_1 + point_2) == point_3)
    }

    #[test]
    fn add_vector_to_vector() {
        let point_1 = Tuple::new_vector(1.0, 2.0, 3.0);
        let point_2 = Tuple::new_vector(1.0, 2.0, 3.0);

        let point_3 = Tuple::new_vector(2.0, 4.0, 6.0);

        assert!((point_1 + point_2) == point_3)
    }

    #[test]
    fn sub_point_to_point() {
        let point_1 = Tuple::new_point(3.0, 2.0, 1.0);
        let point_2 = Tuple::new_point(5.0, 6.0, 7.0);

        let point_3 = Tuple::new_vector(-2.0, -4.0, -6.0);

        assert!((point_1 - point_2) == point_3)
    }

    #[test]
    fn sub_vector_to_point() {
        let point_1 = Tuple::new_point(3.0, 2.0, 1.0);
        let point_2 = Tuple::new_vector(5.0, 6.0, 7.0);

        let point_3 = Tuple::new_point(-2.0, -4.0, -6.0);

        assert!((point_1 - point_2) == point_3)
    }

    #[test]
    fn sub_vector_to_vector() {
        let point_1 = Tuple::new_vector(3.0, 2.0, 1.0);
        let point_2 = Tuple::new_vector(5.0, 6.0, 7.0);

        let point_3 = Tuple::new_vector(-2.0, -4.0, -6.0);

        assert!((point_1 - point_2) == point_3)
    }

    #[test]
    fn negate_tuple() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let expected = Tuple::new(-1.0, 2.0, -3.0, 4.0);

        assert!(-tuple == expected);
    }

    #[test]
    fn multiply_tuple_by_scalar() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let scalar: f64 = 3.5;

        let expected = Tuple::new(3.5, -7.0, 10.5, -14.0);
        assert!(tuple * scalar == expected);
    }

    #[test]
    fn multiply_tuple_by_fraction() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let scalar: f64 = 0.5;

        let expected = Tuple::new(0.5, -1.0, 1.5, -2.0);
        assert!(tuple * scalar == expected);
    }

    #[test]
    fn divide_tuple_by_scalar() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let scalar: f64 = 2.0;

        let expected = Tuple::new(0.5, -1.0, 1.5, -2.0);
        assert!(tuple / scalar == expected);
    }

    #[test]
    fn magnitude_computation() {
        assert!(1.0 == Tuple::new_vector(1.0, 0.0, 0.0).magnitude());
        assert!(1.0 == Tuple::new_vector(0.0, 1.0, 0.0).magnitude());
        assert!(1.0 == Tuple::new_vector(0.0, 0.0, 1.0).magnitude());
        assert!((14.0_f64).sqrt() == Tuple::new_vector(1.0, 2.0, 3.0).magnitude());
        assert!((14.0_f64).sqrt() == Tuple::new_vector(-1.0, -2.0, -3.0).magnitude());
    }

    #[test]
    fn normalize_computation() {
        assert!(Tuple::new_vector(1.0, 0.0, 0.0) == Tuple::new_vector(4.0, 0.0, 0.0).normalize());
    }

    #[test]
    fn normalize_vector_has_magnitude_1() {
        let outcome = Tuple::new_vector(1.0, 2.0, 3.0).normalize();
        let expected = Tuple::new_vector(
            1.0 / (14.0_f64).sqrt(),
            2.0 / (14.0_f64).sqrt(),
            3.0 / (14.0_f64).sqrt(),
        );
        assert!(outcome == expected);
        assert!(outcome.magnitude().approx_eq(1.0, F64Margin::default()));
    }

    #[test]
    fn dot_product_between_vectors() {
        let vector_1 = Tuple::new_vector(1.0, 2.0, 3.0);
        let vector_2 = Tuple::new_vector(2.0, 3.0, 4.0);

        assert!(vector_1.dot(&vector_2) == 20.0);
    }

    #[test]
    fn cross_product_between_vectors() {
        let vector_1 = Tuple::new_vector(1.0, 2.0, 3.0);
        let vector_2 = Tuple::new_vector(2.0, 3.0, 4.0);

        let expected = Tuple::new_vector(-1.0, 2.0, -1.0);

        assert!(vector_1.cross(&vector_2) == expected);
    }

    #[test]
    fn cross_product_properties() {
        let vector_x = Tuple::new_vector(1.0, 0.0, 0.0);
        let vector_y = Tuple::new_vector(0.0, 1.0, 0.0);
        let vector_z = Tuple::new_vector(0.0, 0.0, 1.0);

        assert!(vector_x.cross(&vector_y) == vector_z);
        assert!(vector_y.cross(&vector_z) == vector_x);

        assert!(vector_y.cross(&vector_x) == -vector_z);
    }

    #[test]
    fn add_colors() {
        let color_1 = Tuple::new_color(0.9, 0.6, 0.75);
        let color_2 = Tuple::new_color(0.7, 0.1, 0.25);

        let expected = Tuple::new_color(1.6, 0.7, 1.0);
        assert!((color_1 + color_2) == expected);
    }

    #[test]
    fn subtract_colors() {
        let color_1 = Tuple::new_color(0.9, 0.6, 0.75);
        let color_2 = Tuple::new_color(0.7, 0.1, 0.25);

        let expected = Tuple::new_color(0.2, 0.5, 0.5);
        assert!(color_1 - color_2 == expected);
    }

    #[test]
    fn multiply_color_by_scalar() {
        let color_1 = Tuple::new_color(0.2, 0.3, 0.4);
        let scalar = 2.0;

        let expected = Tuple::new_color(0.4, 0.6, 0.8);
        assert!(color_1 * scalar == expected);
    }

    #[test]
    fn multiply_colors() {
        let color_1 = Tuple::new_color(1.0, 0.2, 0.4);
        let color_2 = Tuple::new_color(0.9, 1.0, 0.1);

        let expected = Tuple::new_color(0.9, 0.2, 0.04);
        assert!(color_1.hadamard_product(&color_2) == expected);
    }

    #[test]
    fn reflect_a_vector_at_45_degrees() {
        let v = Tuple::new_vector(1.0, -1.0, 0.0);
        let n = Tuple::new_vector(0.0, 1.0, 0.0);
        let r = v.reflect(&n);

        assert!(r == Tuple::new_vector(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflect_a_vector_over_a_slanted_surface() {
        let v = Tuple::new_vector(0.0, -1.0, 0.0);
        let n = Tuple::new_vector(2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);
        let r = v.reflect(&n);

        assert!(r == Tuple::new_vector(1.0, 0.0, 0.0));
    }
}
