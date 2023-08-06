use std::ops;

use float_cmp::{ApproxEq, F32Margin};

struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Tuple {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Tuple {
        Tuple { x, y, z, w }
    }

    fn new_point(x: f32, y: f32, z: f32) -> Tuple {
        Tuple { x, y, z, w: 1.0 }
    }

    fn new_vector(x: f32, y: f32, z: f32) -> Tuple {
        Tuple { x, y, z, w: 0.0 }
    }

    fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    fn magnitude(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0) + self.w.powf(2.0)).sqrt()
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        let margin = F32Margin::default();
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

impl ops::Mul<f32> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f32) -> Tuple {
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

impl ops::Div<f32> for Tuple {
    type Output = Self;

    fn div(self, rhs: f32) -> Tuple {
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
        let scalar: f32 = 3.5;

        let expected = Tuple::new(3.5, -7.0, 10.5, -14.0);
        assert!(tuple * scalar == expected);
    }

    #[test]
    fn multiply_tuple_by_fraction() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let scalar: f32 = 0.5;

        let expected = Tuple::new(0.5, -1.0, 1.5, -2.0);
        assert!(tuple * scalar == expected);
    }

    #[test]
    fn divide_tuple_by_scalar() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let scalar: f32 = 2.0;

        let expected = Tuple::new(0.5, -1.0, 1.5, -2.0);
        assert!(tuple / scalar == expected);
    }

    #[test]
    fn magnitude_computation() {
        assert!(1.0 == Tuple::new_vector(1.0, 0.0, 0.0).magnitude());
        assert!(1.0 == Tuple::new_vector(0.0, 1.0, 0.0).magnitude());
        assert!(1.0 == Tuple::new_vector(0.0, 0.0, 1.0).magnitude());
        assert!((14.0_f32).sqrt() == Tuple::new_vector(1.0, 2.0, 3.0).magnitude());
        assert!((14.0_f32).sqrt() == Tuple::new_vector(-1.0, -2.0, -3.0).magnitude());
    }
}
