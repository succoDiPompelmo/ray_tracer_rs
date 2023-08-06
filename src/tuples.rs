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
}
