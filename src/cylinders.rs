use float_cmp::{ApproxEq, F64Margin};

use crate::{rays::Ray, shapes::Polygon, tuples::Tuple};

pub struct Cylinder {}

impl Cylinder {
    pub fn new() -> Cylinder {
        Cylinder {}
    }
}

impl Polygon for Cylinder {
    fn intersect(&self, original_ray: &Ray) -> Vec<f64> {
        let a = original_ray.get_direction().x.powi(2) + original_ray.get_direction().z.powi(2);

        let margin = F64Margin {
            ulps: 2,
            epsilon: 1e-14,
        };

        // ray is parallel to the y axis
        if a.approx_eq(0.0, margin) {
            return vec![];
        }

        let b = 2.0 * original_ray.get_origin().x * original_ray.get_direction().x
            + 2.0 * original_ray.get_origin().z * original_ray.get_direction().z;
        let c = original_ray.get_origin().x.powi(2) + original_ray.get_origin().z.powi(2) - 1.0;

        let disc = b.powi(2) - 4.0 * a * c;

        // ray does not intersect the cylinder
        if disc < 0.0 {
            return vec![];
        }

        let t0 = (-b - disc.sqrt()) / (2.0 * a);
        let t1 = (-b + disc.sqrt()) / (2.0 * a);
        vec![t0, t1]
    }

    fn normal_at(&self, point: &Tuple) -> Tuple {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn a_ray_misses_a_cylinder(origin: Tuple, direction: Tuple) {
        let cyl = Cylinder::new();
        let r = Ray::new(origin, direction.normalize());
        let xs = cyl.intersect(&r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_misses_a_cylinder_scenarios() {
        a_ray_misses_a_cylinder(
            Tuple::new_point(1.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );
        a_ray_misses_a_cylinder(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );
        a_ray_misses_a_cylinder(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(1.0, 1.0, 1.0),
        );
    }

    fn a_ray_strikes_a_cylinder(origin: Tuple, direction: Tuple, t1: f64, t2: f64) {
        let cyl = Cylinder::new();
        let r = Ray::new(origin, direction.normalize());
        let xs = cyl.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(*xs.get(0).unwrap(), t1);
        assert_eq!(*xs.get(1).unwrap(), t2);
    }

    #[test]
    fn a_ray_strikes_a_cylinder_scenarios() {
        a_ray_strikes_a_cylinder(
            Tuple::new_point(1.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
            5.0,
            5.0,
        );
        a_ray_strikes_a_cylinder(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
            4.0,
            6.0,
        );
        a_ray_strikes_a_cylinder(
            Tuple::new_point(0.5, 0.0, -5.0),
            Tuple::new_vector(0.1, 1.0, 1.0),
            6.80798191702732,
            7.088723439378861,
        );
    }
}
