use crate::{matrices::Matrix, shapes::Shape, tuples::Tuple};

#[derive(Clone, Debug)]
pub struct Pattern {
    color_a: Tuple,
    color_b: Tuple,
    transformation: Matrix,
}

impl Pattern {
    pub fn stripe(color_a: Tuple, color_b: Tuple) -> Pattern {
        Pattern {
            color_a,
            color_b,
            transformation: Matrix::identity(4),
        }
    }

    pub fn stripe_at_object(&self, object: Shape, world_point: Tuple) -> Tuple {
        let object_point = &object.get_inverse_transformation() * &world_point;
        let pattern_point = &self.transformation.invert() * &object_point;

        self.stripe_at(&pattern_point)
    }

    pub fn stripe_at(&self, point: &Tuple) -> Tuple {
        if (point.x.floor() as i64) % 2 == 0 {
            return self.color_a;
        }

        self.color_b
    }

    pub fn set_transformation(&mut self, transformation: Matrix) {
        self.transformation = transformation;
    }
}

#[cfg(test)]
mod tests {

    use std::sync::{Arc, Mutex};

    use crate::{shapes::Shape, spheres::Sphere, transformations::Transformation, tuples::Tuple};

    use super::*;

    #[test]
    fn creating_a_stripe_pattern() {
        let pattern = Pattern::stripe(
            Tuple::new_color(1.0, 1.0, 1.0),
            Tuple::new_color(0.0, 0.0, 0.0),
        );

        assert!(Tuple::new_color(1.0, 1.0, 1.0) == pattern.color_a);
        assert!(Tuple::new_color(0.0, 0.0, 0.0) == pattern.color_b);
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = Pattern::stripe(
            Tuple::new_color(1.0, 1.0, 1.0),
            Tuple::new_color(0.0, 0.0, 0.0),
        );

        assert!(
            Tuple::new_color(1.0, 1.0, 1.0) == pattern.stripe_at(&Tuple::new_point(0.0, 0.0, 0.0))
        );
        assert!(
            Tuple::new_color(1.0, 1.0, 1.0) == pattern.stripe_at(&Tuple::new_point(0.0, 1.0, 0.0))
        );
        assert!(
            Tuple::new_color(1.0, 1.0, 1.0) == pattern.stripe_at(&Tuple::new_point(0.0, 2.0, 0.0))
        );
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = Pattern::stripe(
            Tuple::new_color(1.0, 1.0, 1.0),
            Tuple::new_color(0.0, 0.0, 0.0),
        );

        assert!(
            Tuple::new_color(1.0, 1.0, 1.0) == pattern.stripe_at(&Tuple::new_point(0.0, 0.0, 0.0))
        );
        assert!(
            Tuple::new_color(1.0, 1.0, 1.0) == pattern.stripe_at(&Tuple::new_point(0.0, 0.0, 1.0))
        );
        assert!(
            Tuple::new_color(1.0, 1.0, 1.0) == pattern.stripe_at(&Tuple::new_point(0.0, 0.0, 2.0))
        );
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = Pattern::stripe(
            Tuple::new_color(1.0, 1.0, 1.0),
            Tuple::new_color(0.0, 0.0, 0.0),
        );

        assert!(
            Tuple::new_color(1.0, 1.0, 1.0) == pattern.stripe_at(&Tuple::new_point(0.0, 0.0, 0.0))
        );
        assert!(
            Tuple::new_color(1.0, 1.0, 1.0) == pattern.stripe_at(&Tuple::new_point(0.9, 0.0, 1.0))
        );
        assert!(
            Tuple::new_color(0.0, 0.0, 0.0) == pattern.stripe_at(&Tuple::new_point(1.0, 0.0, 2.0))
        );
        assert!(
            Tuple::new_color(0.0, 0.0, 0.0) == pattern.stripe_at(&Tuple::new_point(-0.1, 0.0, 0.0))
        );
        assert!(
            Tuple::new_color(0.0, 0.0, 0.0) == pattern.stripe_at(&Tuple::new_point(-1.0, 0.0, 1.0))
        );
        assert!(
            Tuple::new_color(1.0, 1.0, 1.0) == pattern.stripe_at(&Tuple::new_point(-1.1, 0.0, 2.0))
        );
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        let mut object = Shape::default(Arc::new(Mutex::new(Sphere::new())));
        object.set_transformation(Transformation::scaling(2.0, 2.0, 2.0));

        let pattern = Pattern::stripe(
            Tuple::new_color(1.0, 1.0, 1.0),
            Tuple::new_color(0.0, 0.0, 0.0),
        );

        let c = pattern.stripe_at_object(object, Tuple::new_point(1.5, 0.0, 0.0));

        assert_eq!(Tuple::new_color(1.0, 1.0, 1.0), c);
    }

    #[test]
    fn stripes_with_an_pattern_transformation() {
        let object = Shape::default(Arc::new(Mutex::new(Sphere::new())));

        let mut pattern = Pattern::stripe(
            Tuple::new_color(1.0, 1.0, 1.0),
            Tuple::new_color(0.0, 0.0, 0.0),
        );

        pattern.set_transformation(Transformation::scaling(2.0, 2.0, 2.0));

        let c = pattern.stripe_at_object(object, Tuple::new_point(1.5, 0.0, 0.0));

        assert_eq!(Tuple::new_color(1.0, 1.0, 1.0), c);
    }

    #[test]
    fn stripes_with_both_an_object_and_an_pattern_transformation() {
        let mut object = Shape::default(Arc::new(Mutex::new(Sphere::new())));
        object.set_transformation(Transformation::scaling(2.0, 2.0, 2.0));

        let mut pattern = Pattern::stripe(
            Tuple::new_color(1.0, 1.0, 1.0),
            Tuple::new_color(0.0, 0.0, 0.0),
        );

        pattern.set_transformation(Transformation::translation(0.5, 0.0, 0.0));

        let c = pattern.stripe_at_object(object, Tuple::new_point(2.5, 0.0, 0.0));

        assert_eq!(Tuple::new_color(1.0, 1.0, 1.0), c);
    }
}
