use crate::{matrices::Matrix, tuples::Tuple};

#[derive(Debug, PartialEq)]
pub struct Ray {
    origin: Tuple,
    direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray { origin, direction }
    }

    pub fn get_origin(&self) -> Tuple {
        self.origin
    }

    pub fn get_direction(&self) -> Tuple {
        self.direction
    }

    pub fn position(&self, distance: f64) -> Tuple {
        self.origin + self.direction * distance
    }

    pub fn transform(&self, t: &Matrix) -> Ray {
        Ray {
            origin: t * &self.origin,
            direction: t * &self.direction,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::transformations::Transformation;

    use super::*;

    #[test]
    fn create_a_ray() {
        let origin = Tuple::new_point(1.0, 2.0, 3.0);
        let direction = Tuple::new_vector(4.0, 5.0, 6.0);

        let ray = Ray::new(origin, direction);

        assert!(ray.get_origin() == origin);
        assert!(ray.get_direction() == direction);
    }

    #[test]
    fn compute_point_from_distance() {
        let r = Ray::new(
            Tuple::new_point(2.0, 3.0, 4.0),
            Tuple::new_vector(1.0, 0.0, 0.0),
        );

        assert!(r.position(0.0) == Tuple::new_point(2.0, 3.0, 4.0));
        assert!(r.position(1.0) == Tuple::new_point(3.0, 3.0, 4.0));
        assert!(r.position(-1.0) == Tuple::new_point(1.0, 3.0, 4.0));
        assert!(r.position(2.5) == Tuple::new_point(4.5, 3.0, 4.0));
    }

    #[test]
    fn translate_a_ray() {
        let r = Ray::new(
            Tuple::new_point(1.0, 2.0, 3.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );
        let t = Transformation::translation(3.0, 4.0, 5.0);

        let p = Tuple::new_point(4.0, 6.0, 8.0);
        let v = Tuple::new_vector(0.0, 1.0, 0.0);

        let r2 = r.transform(&t);

        assert!(r2.get_origin() == p);
        assert!(r2.get_direction() == v);
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(
            Tuple::new_point(1.0, 2.0, 3.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );
        let t = Transformation::scaling(2.0, 3.0, 4.0);

        let p = Tuple::new_point(2.0, 6.0, 12.0);
        let v = Tuple::new_vector(0.0, 3.0, 0.0);

        let r2 = r.transform(&t);

        assert!(r2.get_origin() == p);
        assert!(r2.get_direction() == v);
    }
}
