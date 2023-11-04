use float_cmp::ApproxEq;

use crate::{margin::Margin, rays::Ray, shapes::Shape, tuples::Tuple};

#[derive(Clone, Debug)]
pub struct Intersection {
    t: f64,
    object: Shape,
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.t.approx_eq(other.get_t(), Margin::default_f64())
    }
}

#[derive(Debug)]
pub struct Computations {
    _t: f64,
    object: Shape,
    point: Tuple,
    eyev: Tuple,
    normalv: Tuple,
    reflectv: Tuple,
    n1: f64,
    n2: f64,
    _inside: bool,
    over_point: Tuple,
    under_point: Tuple,
}

impl Intersection {
    pub fn new(t: f64, object: Shape) -> Intersection {
        Intersection { t, object }
    }

    #[cfg(test)]
    pub fn intersects(intersections: &[Intersection]) -> Vec<Intersection> {
        intersections.to_vec()
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

    pub fn prepare_computations(&self, ray: &Ray, xs: &[Intersection]) -> Computations {
        let t = self.t;
        let object = self.object.clone();

        let point = ray.position(t);
        let eyev = -ray.get_direction();

        let mut normalv = object.normal_at(&point, None);

        let mut inside = false;

        if normalv.dot(&eyev) < 0.0 {
            inside = true;
            normalv = -normalv
        }

        let reflectv = ray.get_direction().reflect(&normalv);

        let over_point = point + normalv * Computations::get_epsilon();
        let under_point = point - normalv * Computations::get_epsilon();

        let mut containers: Vec<Shape> = vec![];

        let mut n1 = 1.0;
        let mut n2 = 1.0;

        for i in xs {
            if self == i && !containers.is_empty() {
                n1 = containers
                    .last()
                    .unwrap()
                    .get_material()
                    .get_refractive_index();
            }

            if containers.contains(&i.object) {
                containers.retain(|element| &i.object != element);
            } else {
                containers.push(i.object.clone())
            }

            if self == i {
                if !containers.is_empty() {
                    n2 = containers
                        .last()
                        .unwrap()
                        .get_material()
                        .get_refractive_index();
                }

                break;
            }
        }

        Computations {
            _t: t,
            object,
            point,
            eyev,
            normalv,
            reflectv,
            n1,
            n2,
            _inside: inside,
            over_point,
            under_point,
        }
    }
}

impl Computations {
    pub fn get_object(&self) -> Shape {
        self.object.clone()
    }

    pub fn get_point_ref(&self) -> &Tuple {
        &self.point
    }

    pub fn get_eyev_ref(&self) -> &Tuple {
        &self.eyev
    }

    pub fn get_normalv_ref(&self) -> &Tuple {
        &self.normalv
    }

    fn get_epsilon() -> f64 {
        0.000001
    }

    pub fn get_over_point_ref(&self) -> &Tuple {
        &self.over_point
    }

    pub fn get_under_point_ref(&self) -> &Tuple {
        &self.under_point
    }

    pub fn get_reflectv(&self) -> &Tuple {
        &self.reflectv
    }

    pub fn get_n1(&self) -> f64 {
        self.n1
    }

    pub fn get_n2(&self) -> f64 {
        self.n2
    }

    pub fn schlick(&self) -> f64 {
        let mut cos = self.eyev.dot(&self.normalv);

        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n.powi(2) * (1.0 - cos.powi(2));

            if sin2_t > 1.0 {
                return 1.0;
            }

            let cos_t = (1.0 - sin2_t).sqrt();
            cos = cos_t;
        }

        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

#[cfg(test)]
mod tests {

    use std::sync::{Arc, Mutex};

    use crate::{
        planes::Plane, rays::Ray, shapes::Shape, spheres::Sphere, transformations::Transformation,
        tuples::Tuple,
    };

    use super::*;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let sphere = Sphere::new();
        let s = Shape::default(Arc::new(Mutex::new(sphere)));
        let t = 3.5;

        let intersection = Intersection::new(t, s.clone());

        assert!(intersection.get_t() == t);
    }

    #[test]
    fn aggregate_intersections() {
        let sphere = Sphere::new();
        let s = Shape::default(Arc::new(Mutex::new(sphere)));

        let i1 = Intersection::new(1.0, s.clone());
        let i2 = Intersection::new(2.0, s);

        let xs = Intersection::intersects(&[i1, i2]);

        assert!(xs.len() == 2);
        assert!(xs.get(0).unwrap().t == 1.0);
        assert!(xs.get(1).unwrap().t == 2.0);
    }

    #[test]
    fn hit_when_all_intersections_are_positives() {
        let sphere = Sphere::new();
        let s = Shape::default(Arc::new(Mutex::new(sphere)));

        let i1 = Intersection::new(1.0, s.clone());
        let i2 = Intersection::new(2.0, s);

        let xs = Intersection::intersects(&[i1.clone(), i2]);

        assert!(Intersection::hit(&xs) == Some(i1));
    }

    #[test]
    fn hit_when_some_intersections_are_negatives() {
        let sphere = Sphere::new();
        let s = Shape::default(Arc::new(Mutex::new(sphere)));

        let i1 = Intersection::new(-1.0, s.clone());
        let i2 = Intersection::new(1.0, s);

        let xs = Intersection::intersects(&[i1, i2.clone()]);

        assert!(Intersection::hit(&xs) == Some(i2));
    }

    #[test]
    fn hit_when_all_intersections_are_negatives() {
        let sphere = Sphere::new();
        let s = Shape::default(Arc::new(Mutex::new(sphere)));

        let i1 = Intersection::new(-2.0, s.clone());
        let i2 = Intersection::new(-1.0, s);

        let xs = Intersection::intersects(&[i1, i2]);

        assert!(Intersection::hit(&xs) == None);
    }

    #[test]
    fn hit_is_always_the_lowest_nonnegative_intersection() {
        let sphere = Sphere::new();
        let s = Shape::default(Arc::new(Mutex::new(sphere)));

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

        let sphere = Sphere::new();
        let s = Shape::default(Arc::new(Mutex::new(sphere)));

        let i = Intersection::new(4.0, s);

        let comps = i.prepare_computations(&r, &[]);

        assert!(comps._t == i.t);
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
        let sphere = Sphere::new();
        let s = Shape::default(Arc::new(Mutex::new(sphere)));

        let i = Intersection::new(4.0, s);

        let comps = i.prepare_computations(&r, &[]);
        assert!(!comps._inside);
    }

    #[test]
    fn the_hit_when_an_intersection_occours_on_the_inside() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let sphere = Sphere::new();
        let s = Shape::default(Arc::new(Mutex::new(sphere)));

        let i = Intersection::new(1.0, s);

        let comps = i.prepare_computations(&r, &[]);

        assert!(comps.point == Tuple::new_point(0.0, 0.0, 1.0));
        assert!(comps.eyev == Tuple::new_vector(0.0, 0.0, -1.0));
        assert!(comps._inside);
        assert!(comps.normalv == Tuple::new_vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let sphere = Sphere::new();
        let mut s = Shape::default(Arc::new(Mutex::new(sphere)));

        s.set_transformation(Transformation::translation(0.0, 0.0, 1.0));

        let i = Intersection::new(5.0, s);
        let comps = i.prepare_computations(&r, &[]);

        assert!(comps.over_point.z < -Computations::get_epsilon() / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn precomputing_the_reflection_vector() {
        let plane = Plane::new();
        let s = Shape::default(Arc::new(Mutex::new(plane)));

        let r = Ray::new(
            Tuple::new_point(0.0, 1.0, -1.0),
            Tuple::new_vector(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0_f64.sqrt(), s);

        let comps = i.prepare_computations(&r, &[]);

        assert_eq!(
            comps.reflectv,
            Tuple::new_vector(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn the_under_point_is_offset_below_the_surface() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let mut s = Shape::glass(Arc::new(Mutex::new(Sphere::new())));

        let transform = Transformation::translation(0.0, 0.0, 1.0);
        s.set_transformation(transform);

        let i = Intersection::new(5.0, s);
        let xs = Intersection::intersects(&[i.clone()]);

        let comps = i.prepare_computations(&r, &xs);
        assert!(comps.under_point.z > 0.0);
        assert!(comps.point.z < comps.under_point.z);
    }

    #[test]
    fn the_schlick_approximation_under_total_internal_reflection() {
        let shape = Shape::glass(Arc::new(Mutex::new(Sphere::new())));
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 2.0_f64.sqrt() / 2.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );
        let xs = Intersection::intersects(&[
            Intersection::new(-2.0_f64.sqrt() / 2.0, shape.clone()),
            Intersection::new(2.0_f64.sqrt() / 2.0, shape.clone()),
        ]);

        let comps: Computations = xs.get(1).unwrap().prepare_computations(&r, &xs);
        let reflectance = comps.schlick();

        assert!(reflectance.approx_eq(1.0, Margin::default_f64()));
    }

    #[test]
    fn determine_reflectance_of_a_perpendicular_ray() {
        let shape = Shape::glass(Arc::new(Mutex::new(Sphere::new())));
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );
        let xs = Intersection::intersects(&[
            Intersection::new(-1.0, shape.clone()),
            Intersection::new(1.0, shape.clone()),
        ]);

        let comps: Computations = xs.get(1).unwrap().prepare_computations(&r, &xs);
        let reflectance = comps.schlick();

        assert!(reflectance.approx_eq(0.04, Margin::default_f64()));
    }

    #[test]
    fn the_schlick_approximation_with_small_angle_and_n2_greater_than_n1() {
        let shape = Shape::glass(Arc::new(Mutex::new(Sphere::new())));
        let r = Ray::new(
            Tuple::new_point(0.0, 0.99, -2.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let xs = Intersection::intersects(&[Intersection::new(1.8589, shape.clone())]);

        let comps: Computations = xs.get(0).unwrap().prepare_computations(&r, &xs);
        let reflectance = comps.schlick();

        assert!(reflectance.approx_eq(0.48873081012212183, Margin::default_f64()));
    }
}
