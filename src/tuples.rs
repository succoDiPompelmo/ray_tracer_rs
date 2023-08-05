use float_cmp::{ApproxEq, F32Margin};

struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: TupleType,
}

#[derive(PartialEq)]
enum TupleType {
    Point,
    Vector,
}

impl Tuple {
    fn new_point(x: f32, y: f32, z: f32) -> Tuple {
        Tuple {
            x,
            y,
            z,
            w: TupleType::Point,
        }
    }

    fn new_vector(x: f32, y: f32, z: f32) -> Tuple {
        Tuple {
            x,
            y,
            z,
            w: TupleType::Vector,
        }
    }

    fn is_vector(&self) -> bool {
        match self.w {
            TupleType::Vector => true,
            _ => false,
        }
    }

    fn is_point(&self) -> bool {
        match self.w {
            TupleType::Point => true,
            _ => false,
        }
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
}
