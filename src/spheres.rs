use float_cmp::ApproxEq;

use crate::{margin::Margin, rays::Ray, shapes::Polygon, tuples::Tuple};

#[derive(Clone, Debug)]
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
}

impl Polygon for Sphere {
    fn intersect(&self, ray: &Ray) -> Vec<f64> {
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

        vec![t1, t2]
    }

    fn normal_at(&self, object_point: &Tuple) -> Tuple {
        object_point - &self.center
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && self.radius.approx_eq(other.radius, Margin::default_f64())
    }
}

#[cfg(test)]
mod tests {

    use std::{
        f64::consts::PI,
        sync::{Arc, Mutex},
    };

    use crate::{rays::Ray, shapes::Shape, transformations::Transformation, tuples::Tuple};

    use super::*;

    #[test]
    fn ray_intersect_spheres_in_two_points() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let sphere = Sphere::new();
        let s = Shape::default(Arc::new(Mutex::new(sphere)));

        let xs = s.intersect(&r);

        assert!(xs.len() == 2);
        assert!(xs.get(0).unwrap().get_t() == 4.0);
        assert!(xs.get(1).unwrap().get_t() == 6.0);
    }

    #[test]
    fn ray_tangent_to_sphere() {
        let r = Ray::new(
            Tuple::new_point(0.0, 1.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let sphere = Sphere::new();
        let s = Shape::default(Arc::new(Mutex::new(sphere)));

        let xs = s.intersect(&r);

        assert!(xs.len() == 2);
        assert!(xs.get(0).unwrap().get_t() == 5.0);
        assert!(xs.get(1).unwrap().get_t() == 5.0);
    }

    #[test]
    fn ray_miss_the_sphere() {
        let r = Ray::new(
            Tuple::new_point(0.0, 2.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let sphere = Sphere::new();
        let s = Shape::default(Arc::new(Mutex::new(sphere)));

        let xs = s.intersect(&r);

        assert!(xs.len() == 0);
    }

    #[test]
    fn ray_originates_inside_the_sphere() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let sphere = Sphere::new();
        let s = Shape::default(Arc::new(Mutex::new(sphere)));

        let xs = s.intersect(&r);

        assert!(xs.len() == 2);
        assert!(xs.get(0).unwrap().get_t() == -1.0);
        assert!(xs.get(1).unwrap().get_t() == 1.0);
    }

    #[test]
    fn sphere_behind_a_ray() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let sphere = Sphere::new();
        let s = Shape::default(Arc::new(Mutex::new(sphere)));

        let xs = s.intersect(&r);

        assert!(xs.len() == 2);
        assert!(xs.get(0).unwrap().get_t() == -6.0);
        assert!(xs.get(1).unwrap().get_t() == -4.0);
    }

    #[test]
    fn intesecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let sphere = Sphere::new();
        let mut s = Shape::default(Arc::new(Mutex::new(sphere)));

        let t = Transformation::scaling(2.0, 2.0, 2.0);
        s.set_transformation(t.clone());

        let xs = s.intersect(&r);

        assert!(xs.len() == 2);
        assert!(xs.get(0).unwrap().get_t() == 3.0);
        assert!(xs.get(1).unwrap().get_t() == 7.0);
    }

    #[test]
    fn intesecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let sphere = Sphere::new();
        let mut s = Shape::default(Arc::new(Mutex::new(sphere)));

        let t = Transformation::translation(5.0, 0.0, 0.0);
        s.set_transformation(t.clone());

        let xs = s.intersect(&r);

        assert!(xs.len() == 0);
    }

    #[test]
    fn normal_on_a_sphere() {
        let sphere = Sphere::new();
        let s = Shape::default(Arc::new(Mutex::new(sphere)));

        let v1 = Tuple::new_vector(1.0, 0.0, 0.0);
        assert!(s.normal_at(&Tuple::new_point(1.0, 0.0, 0.0)) == v1);

        let v2 = Tuple::new_vector(0.0, 1.0, 0.0);
        assert!(s.normal_at(&Tuple::new_point(0.0, 1.0, 0.0)) == v2);

        let v3 = Tuple::new_vector(0.0, 0.0, 1.0);
        assert!(s.normal_at(&Tuple::new_point(0.0, 0.0, 1.0)) == v3);

        let value = 3.0_f64.sqrt() / 3.0;
        let v4 = Tuple::new_vector(value, value, value);
        assert!(s.normal_at(&Tuple::new_point(value, value, value)) == v4);
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let sphere = Sphere::new();
        let s = Shape::default(Arc::new(Mutex::new(sphere)));

        let value = 3.0_f64.sqrt() / 3.0;
        let n = s.normal_at(&Tuple::new_point(value, value, value));
        assert!(n.normalize() == n);
    }

    #[test]
    fn normal_on_a_translated_sphere() {
        let sphere = Sphere::new();
        let mut s = Shape::default(Arc::new(Mutex::new(sphere)));

        s.set_transformation(Transformation::translation(0.0, 1.0, 0.0));
        let n = s.normal_at(&Tuple::new_point(0.0, 1.70711, -0.70711));

        assert!(n == Tuple::new_vector(0.0, 0.7071067811865475, -0.7071067811865476))
    }

    #[test]
    fn normal_on_a_transformed_sphere() {
        let sphere = Sphere::new();
        let mut s = Shape::default(Arc::new(Mutex::new(sphere)));

        s.set_transformation(
            Transformation::scaling(1.0, 0.5, 1.0) * Transformation::rotation_z(PI / 5.0),
        );
        let n = s.normal_at(&Tuple::new_point(
            0.0,
            2.0_f64.sqrt() / 2.0,
            -2.0_f64.sqrt() / 2.0,
        ));

        assert!(n == Tuple::new_vector(0.0, 0.9701425001453319, -0.24253562503633294))
    }
}
