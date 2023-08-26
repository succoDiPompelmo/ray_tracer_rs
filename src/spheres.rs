use float_cmp::{ApproxEq, F64Margin};

use crate::{intersections::Intersection, rays::Ray, tuples::Tuple};

#[derive(Clone, Copy)]
pub struct Sphere {
    center: Tuple,
    radius: f64,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            center: Tuple::new_point(0.0, 0.0, 0.0),
            radius: 1.0,
        }
    }

    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.get_origin() - self.center;

        let a = ray.get_direction().dot(&ray.get_direction());
        let b = 2.0 * ray.get_direction().dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![];
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        Intersection::intersects(&[Intersection::new(t1, *self), Intersection::new(t2, *self)])
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        let margin = F64Margin {
            ulps: 2,
            epsilon: 1e-14,
        };

        self.center == other.center && self.radius.approx_eq(other.radius, margin)
    }
}

#[cfg(test)]
mod tests {

    use crate::{rays::Ray, tuples::Tuple};

    use super::*;

    #[test]
    fn ray_intersect_spheres_in_two_points() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert!(xs.len() == 2);
        assert!(xs.get(0).unwrap().get_object() == s);
        assert!(xs.get(1).unwrap().get_object() == s);
        assert!(xs.get(0).unwrap().get_t() == 4.0);
        assert!(xs.get(1).unwrap().get_t() == 6.0);
    }

    #[test]
    fn ray_tangent_to_sphere() {
        let r = Ray::new(
            Tuple::new_point(0.0, 1.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert!(xs.len() == 2);
        assert!(xs.get(0).unwrap().get_object() == s);
        assert!(xs.get(1).unwrap().get_object() == s);
        assert!(xs.get(0).unwrap().get_t() == 5.0);
        assert!(xs.get(1).unwrap().get_t() == 5.0);
    }

    #[test]
    fn ray_miss_the_sphere() {
        let r = Ray::new(
            Tuple::new_point(0.0, 2.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert!(xs.len() == 0);
    }

    #[test]
    fn ray_originates_inside_the_sphere() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert!(xs.len() == 2);
        assert!(xs.get(0).unwrap().get_object() == s);
        assert!(xs.get(1).unwrap().get_object() == s);
        assert!(xs.get(0).unwrap().get_t() == -1.0);
        assert!(xs.get(1).unwrap().get_t() == 1.0);
    }

    #[test]
    fn sphere_behind_a_ray() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert!(xs.len() == 2);
        assert!(xs.get(0).unwrap().get_object() == s);
        assert!(xs.get(1).unwrap().get_object() == s);
        assert!(xs.get(0).unwrap().get_t() == -6.0);
        assert!(xs.get(1).unwrap().get_t() == -4.0);
    }
}
