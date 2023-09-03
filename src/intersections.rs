use std::f64::EPSILON;

use crate::{rays::Ray, spheres::Sphere, tuples::Tuple};

#[derive(Clone, Debug, PartialEq)]
pub struct Intersection {
    t: f64,
    object: Sphere,
}

#[derive(Debug)]
pub struct Computations {
    t: f64,
    object: Sphere,
    point: Tuple,
    eyev: Tuple,
    normalv: Tuple,
    inside: bool,
    over_point: Tuple,
}

impl Intersection {
    pub fn new(t: f64, object: Sphere) -> Intersection {
        Intersection { t, object }
    }

    pub fn intersects(intersections: &[Intersection]) -> Vec<Intersection> {
        intersections.to_vec()
    }

    pub fn get_object(&self) -> Sphere {
        self.object.clone()
    }

    pub fn get_t(&self) -> f64 {
        self.t
    }

    pub fn hit(intersections: &[Intersection]) -> Option<Intersection> {
        let mut hit = None;

        for intersection in intersections {
            if intersection.get_t() > 0.0 {
                if hit.is_none() {
                    hit = Some(intersection);
                }

                if let Some(hit_intersection) = hit {
                    if hit_intersection.get_t() > intersection.get_t() {
                        hit = Some(intersection)
                    }
                }
            }
        }

        hit.cloned()
    }

    pub fn prepare_computations(&self, ray: &Ray) -> Computations {
        let t = self.t;
        let object = self.object.clone();

        let point = ray.position(t);
        let eyev = -ray.get_direction();
        let mut normalv = object.normal_at(point);

        let mut inside = false;

        if normalv.dot(&eyev) < 0.0 {
            inside = true;
            normalv = -normalv
        }

        let over_point = point + normalv * Computations::get_epsilon();

        Computations {
            t,
            object,
            point,
            eyev,
            normalv,
            inside,
            over_point,
        }
    }
}

impl Computations {
    pub fn get_object(&self) -> Sphere {
        self.object.clone()
    }

    pub fn get_point(&self) -> Tuple {
        self.point
    }

    pub fn get_eyev(&self) -> Tuple {
        self.eyev
    }

    pub fn get_normalv(&self) -> Tuple {
        self.normalv
    }

    fn get_epsilon() -> f64 {
        EPSILON * 100.0
    }

    pub fn get_over_point(&self) -> Tuple {
        self.over_point
    }
}

#[cfg(test)]
mod tests {

    use crate::{rays::Ray, transformations::Transformation, tuples::Tuple};

    use super::*;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let t = 3.5;

        let intersection = Intersection::new(t, s.clone());

        assert!(intersection.get_object() == s);
        assert!(intersection.get_t() == t);
    }

    #[test]
    fn aggregate_intersections() {
        let s = Sphere::new();

        let i1 = Intersection::new(1.0, s.clone());
        let i2 = Intersection::new(2.0, s);

        let xs = Intersection::intersects(&[i1, i2]);

        assert!(xs.len() == 2);
        assert!(xs.get(0).unwrap().t == 1.0);
        assert!(xs.get(1).unwrap().t == 2.0);
    }

    #[test]
    fn hit_when_all_intersections_are_positives() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, s.clone());
        let i2 = Intersection::new(2.0, s);

        let xs = Intersection::intersects(&[i1.clone(), i2]);

        assert!(Intersection::hit(&xs) == Some(i1));
    }

    #[test]
    fn hit_when_some_intersections_are_negatives() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, s.clone());
        let i2 = Intersection::new(1.0, s);

        let xs = Intersection::intersects(&[i1, i2.clone()]);

        assert!(Intersection::hit(&xs) == Some(i2));
    }

    #[test]
    fn hit_when_all_intersections_are_negatives() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, s.clone());
        let i2 = Intersection::new(-1.0, s);

        let xs = Intersection::intersects(&[i1, i2]);

        assert!(Intersection::hit(&xs) == None);
    }

    #[test]
    fn hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, s.clone());
        let i2 = Intersection::new(7.0, s.clone());
        let i3 = Intersection::new(-3.0, s.clone());
        let i4 = Intersection::new(2.0, s.clone());

        let xs = Intersection::intersects(&[i1, i2, i3, i4.clone()]);

        assert!(Intersection::hit(&xs) == Some(i4));
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );

        let shape = Sphere::new();
        let i = Intersection::new(4.0, shape);

        let comps = i.prepare_computations(&r);

        assert!(comps.t == i.t);
        assert!(comps.point == Tuple::new_point(0.0, 0.0, -1.0));
        assert!(comps.eyev == Tuple::new_vector(0.0, 0.0, -1.0));
        assert!(comps.normalv == Tuple::new_vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_when_an_intersection_occours_on_the_outside() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let shape = Sphere::new();

        let i = Intersection::new(4.0, shape);

        let comps = i.prepare_computations(&r);
        assert!(!comps.inside);
    }

    #[test]
    fn the_hit_when_an_intersection_occours_on_the_inside() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let shape = Sphere::new();

        let i = Intersection::new(1.0, shape);

        let comps = i.prepare_computations(&r);

        assert!(comps.point == Tuple::new_point(0.0, 0.0, 1.0));
        assert!(comps.eyev == Tuple::new_vector(0.0, 0.0, -1.0));
        assert!(comps.inside);
        assert!(comps.normalv == Tuple::new_vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let mut shape = Sphere::new();
        shape.set_transformation(Transformation::translation(0.0, 0.0, 1.0));

        let i = Intersection::new(5.0, shape);
        let comps = i.prepare_computations(&r);

        assert!(comps.over_point.z < -Computations::get_epsilon() / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
}
