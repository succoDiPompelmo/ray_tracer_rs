use crate::{matrices::Matrix, shapes::Shape, tuples::Tuple};

#[derive(Clone, Debug)]
pub enum PatternsKind {
    Stripe,
    Gradient,
    Ring,
    Checker,
}

#[derive(Clone, Debug)]
pub struct Pattern {
    color_a: Tuple,
    color_b: Tuple,
    transformation: Matrix,
    kind: PatternsKind,
}

impl Pattern {
    pub fn stripe(color_a: Tuple, color_b: Tuple, kind: PatternsKind) -> Pattern {
        Pattern {
            color_a,
            color_b,
            transformation: Matrix::identity(4),
            kind,
        }
    }

    pub fn stripe_at_object(&self, object: &Shape, world_point: &Tuple) -> Tuple {
        let object_point = &object.get_inverse_transformation() * world_point;
        let pattern_point = &self.transformation.invert() * &object_point;

        self.stripe_at(&pattern_point)
    }

    pub fn stripe_at(&self, point: &Tuple) -> Tuple {
        match self.kind {
            PatternsKind::Stripe => {
                if (point.x.floor() as i64) % 2 == 0 {
                    return self.color_a;
                }
                self.color_b
            }
            PatternsKind::Gradient => {
                let distance = self.color_b - self.color_a;
                let fraction = point.x - point.x.floor();

                self.color_a + distance * fraction
            }
            PatternsKind::Ring => {
                if (point.x.powi(2) + point.z.powi(2)).sqrt().floor() as i64 % 2 == 0 {
                    return self.color_a;
                }
                self.color_b
            }
            PatternsKind::Checker => {
                if (point.x.abs() + point.y.abs() + point.z.abs()).floor() as i64 % 2 == 0 {
                    return self.color_a;
                }
                self.color_b
            }
        }
    }

    #[cfg(test)]
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
        let pattern = Pattern::stripe(Tuple::white(), Tuple::black(), PatternsKind::Stripe);

        assert_eq!(pattern.color_a, Tuple::white());
        assert_eq!(pattern.color_b, Tuple::black());
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = Pattern::stripe(Tuple::white(), Tuple::black(), PatternsKind::Stripe);

        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(0.0, 0.0, 0.0)),
            Tuple::white()
        );
        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(0.0, 1.0, 0.0)),
            Tuple::white()
        );
        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(0.0, 2.0, 0.0)),
            Tuple::white()
        );
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = Pattern::stripe(Tuple::white(), Tuple::black(), PatternsKind::Stripe);

        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(0.0, 0.0, 0.0)),
            Tuple::white()
        );
        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(0.0, 0.0, 1.0)),
            Tuple::white()
        );
        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(0.0, 0.0, 2.0)),
            Tuple::white()
        );
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = Pattern::stripe(Tuple::white(), Tuple::black(), PatternsKind::Stripe);

        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(0.0, 0.0, 0.0)),
            Tuple::white()
        );
        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(0.9, 0.0, 1.0)),
            Tuple::white()
        );
        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(1.0, 0.0, 2.0)),
            Tuple::black()
        );
        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(-0.1, 0.0, 0.0)),
            Tuple::black()
        );
        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(-1.0, 0.0, 1.0)),
            Tuple::black()
        );
        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(-1.1, 0.0, 2.0)),
            Tuple::white()
        );
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        let mut object = Shape::default(Arc::new(Mutex::new(Sphere::new())));
        object.set_transformation(Transformation::scaling(2.0, 2.0, 2.0));

        let pattern = Pattern::stripe(Tuple::white(), Tuple::black(), PatternsKind::Stripe);

        let c = pattern.stripe_at_object(&object, &Tuple::new_point(1.5, 0.0, 0.0));

        assert_eq!(Tuple::white(), c);
    }

    #[test]
    fn stripes_with_an_pattern_transformation() {
        let object = Shape::default(Arc::new(Mutex::new(Sphere::new())));

        let mut pattern = Pattern::stripe(Tuple::white(), Tuple::black(), PatternsKind::Stripe);

        pattern.set_transformation(Transformation::scaling(2.0, 2.0, 2.0));

        let c = pattern.stripe_at_object(&object, &Tuple::new_point(1.5, 0.0, 0.0));

        assert_eq!(Tuple::white(), c);
    }

    #[test]
    fn stripes_with_both_an_object_and_an_pattern_transformation() {
        let mut object = Shape::default(Arc::new(Mutex::new(Sphere::new())));
        object.set_transformation(Transformation::scaling(2.0, 2.0, 2.0));

        let mut pattern = Pattern::stripe(Tuple::white(), Tuple::black(), PatternsKind::Stripe);

        pattern.set_transformation(Transformation::translation(0.5, 0.0, 0.0));

        let c = pattern.stripe_at_object(&object, &Tuple::new_point(2.5, 0.0, 0.0));

        assert_eq!(Tuple::white(), c);
    }

    #[test]
    fn a_gradient_linearly_interpolates_between_colors() {
        let pattern = Pattern::stripe(Tuple::white(), Tuple::black(), PatternsKind::Gradient);

        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(0.0, 0.0, 0.0)),
            Tuple::white()
        );
        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(0.25, 0.0, 0.0)),
            Tuple::new_color(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(0.5, 0.0, 0.0)),
            Tuple::new_color(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(0.75, 0.0, 0.0)),
            Tuple::new_color(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn a_ring_should_extend_in_both_x_and_z() {
        let pattern = Pattern::stripe(Tuple::white(), Tuple::black(), PatternsKind::Ring);

        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(0.0, 0.0, 0.0)),
            Tuple::white()
        );
        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(1.0, 0.0, 0.0)),
            Tuple::black()
        );
        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(0.0, 0.0, 1.0)),
            Tuple::black()
        );
        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(
                2.0_f64.sqrt() / 2.0,
                0.0,
                2.0_f64.sqrt() / 2.0
            )),
            Tuple::black()
        );
    }

    #[test]
    fn checkers_should_repeat_in_x() {
        let pattern = Pattern::stripe(Tuple::white(), Tuple::black(), PatternsKind::Checker);

        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(0.0, 0.0, 0.0)),
            Tuple::white()
        );
        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(0.99, 0.0, 0.0)),
            Tuple::white()
        );
        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(1.01, 0.0, 0.0)),
            Tuple::black()
        );
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern = Pattern::stripe(Tuple::white(), Tuple::black(), PatternsKind::Checker);

        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(0.0, 0.0, 0.0)),
            Tuple::white()
        );
        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(0.0, 0.99, 0.0)),
            Tuple::white()
        );
        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(0.0, 1.01, 0.0)),
            Tuple::black()
        );
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern = Pattern::stripe(Tuple::white(), Tuple::black(), PatternsKind::Checker);

        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(0.0, 0.0, 0.0)),
            Tuple::white()
        );
        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(0.0, 0.0, 0.99)),
            Tuple::white()
        );
        assert_eq!(
            pattern.stripe_at(&Tuple::new_point(0.0, 0.0, 1.01)),
            Tuple::black()
        );
    }
}
