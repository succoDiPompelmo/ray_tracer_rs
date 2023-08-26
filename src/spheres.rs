use crate::{rays::Ray, tuples::Tuple};

pub struct Sphere {
    center: Tuple,
    radius: f64,
}

impl Sphere {
    fn new() -> Sphere {
        Sphere {
            center: Tuple::new_point(0.0, 0.0, 0.0),
            radius: 1.0,
        }
    }

    fn intersect(&self, ray: Ray) -> Vec<f64> {
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

        let xs: Vec<f64> = s.intersect(r);

        assert!(xs.len() == 2);
        assert!(*xs.get(0).unwrap() == 4.0);
        assert!(*xs.get(1).unwrap() == 6.0);
    }

    #[test]
    fn ray_tangent_to_sphere() {
        let r = Ray::new(
            Tuple::new_point(0.0, 1.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();

        let xs: Vec<f64> = s.intersect(r);

        assert!(xs.len() == 2);
        assert!(*xs.get(0).unwrap() == 5.0);
        assert!(*xs.get(1).unwrap() == 5.0);
    }

    #[test]
    fn ray_miss_the_sphere() {
        let r = Ray::new(
            Tuple::new_point(0.0, 2.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();

        let xs: Vec<f64> = s.intersect(r);

        assert!(xs.len() == 0);
    }

    #[test]
    fn ray_originates_inside_the_sphere() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();

        let xs: Vec<f64> = s.intersect(r);

        assert!(xs.len() == 2);
        assert!(*xs.get(0).unwrap() == -1.0);
        assert!(*xs.get(1).unwrap() == 1.0);
    }

    #[test]
    fn sphere_behind_a_ray() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();

        let xs: Vec<f64> = s.intersect(r);

        assert!(xs.len() == 2);
        assert!(*xs.get(0).unwrap() == -6.0);
        assert!(*xs.get(1).unwrap() == -4.0);
    }
}
