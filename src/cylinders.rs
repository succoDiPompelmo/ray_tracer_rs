use float_cmp::{ApproxEq, F64Margin};

use crate::{rays::Ray, shapes::Polygon, tuples::Tuple};

pub struct Cylinder {
    minimum: f64,
    maximum: f64,
    closed: bool,
}

impl Cylinder {
    #[cfg(test)]
    pub fn new() -> Cylinder {
        Cylinder {
            minimum: f64::NEG_INFINITY,
            maximum: f64::INFINITY,
            closed: false,
        }
    }

    fn intersect_caps(&self, ray: &Ray) -> Vec<f64> {
        let margin = F64Margin {
            ulps: 2,
            epsilon: 1e-14,
        };

        if !self.closed || ray.get_direction().y.approx_eq(0.0, margin) {
            return vec![];
        }

        let mut xs = vec![];

        let t1 = (self.minimum - ray.get_origin().y) / ray.get_direction().y;
        if check_cap(ray, t1) {
            xs.push(t1);
        }

        let t2 = (self.maximum - ray.get_origin().y) / ray.get_direction().y;
        if check_cap(ray, t2) {
            xs.push(t2);
        }

        xs
    }
}

impl Polygon for Cylinder {
    fn intersect(&self, original_ray: &Ray) -> Vec<f64> {
        let a = original_ray.get_direction().x.powi(2) + original_ray.get_direction().z.powi(2);

        let margin = F64Margin {
            ulps: 2,
            epsilon: 1e-14,
        };

        let mut xs = vec![];

        // ray is parallel to the y axis
        if !a.approx_eq(0.0, margin) {
            let b = 2.0 * original_ray.get_origin().x * original_ray.get_direction().x
                + 2.0 * original_ray.get_origin().z * original_ray.get_direction().z;
            let c = original_ray.get_origin().x.powi(2) + original_ray.get_origin().z.powi(2) - 1.0;

            let disc = b.powi(2) - 4.0 * a * c;

            // ray does not intersect the cylinder
            if disc < 0.0 {
                return vec![];
            }

            let mut t0 = (-b - disc.sqrt()) / (2.0 * a);
            let mut t1 = (-b + disc.sqrt()) / (2.0 * a);

            (t0, t1) = if t0 > t1 { (t1, t0) } else { (t0, t1) };

            let y0 = original_ray.get_origin().y + t0 * original_ray.get_direction().y;
            if self.minimum < y0 && y0 < self.maximum {
                xs.push(t0);
            }

            let y1 = original_ray.get_origin().y + t1 * original_ray.get_direction().y;
            if self.minimum < y1 && y1 < self.maximum {
                xs.push(t1)
            }
        }

        let mut xs_caps = self.intersect_caps(original_ray);
        xs.append(&mut xs_caps);

        xs
    }

    fn normal_at(&self, point: &Tuple) -> Tuple {
        Tuple::new_vector(point.x, 0.0, point.z)
    }
}

fn check_cap(ray: &Ray, t: f64) -> bool {
    let x = ray.get_origin().x + t * ray.get_direction().x;
    let z = ray.get_origin().z + t * ray.get_direction().z;

    let margin = F64Margin {
        ulps: 2,
        epsilon: 1e-14,
    };

    (x.powi(2) + z.powi(2)) < 1.0 || (x.powi(2) + z.powi(2)).approx_eq(1.0, margin)
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

    fn normal_vector_on_a_cylinder(point: Tuple, normal: Tuple) {
        let cyl = Cylinder::new();
        let n = cyl.normal_at(&point);

        assert_eq!(n, normal);
    }

    #[test]
    fn normal_vector_on_a_cylinder_scenarios() {
        normal_vector_on_a_cylinder(
            Tuple::new_point(1.0, 0.0, 0.0),
            Tuple::new_vector(1.0, 0.0, 0.0),
        );
        normal_vector_on_a_cylinder(
            Tuple::new_point(0.0, 5.0, -1.0),
            Tuple::new_vector(0.0, 0.0, -1.0),
        );
        normal_vector_on_a_cylinder(
            Tuple::new_point(0.0, -2.0, 1.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        normal_vector_on_a_cylinder(
            Tuple::new_point(-1.0, 1.0, 0.0),
            Tuple::new_vector(-1.0, 0.0, 0.0),
        );
    }

    #[test]
    fn the_default_minimum_and_maximum_for_a_cylinder() {
        let cyl = Cylinder::new();

        assert_eq!(cyl.minimum, f64::NEG_INFINITY);
        assert_eq!(cyl.maximum, f64::INFINITY);
    }

    fn intersecting_a_constrained_cylinder(point: Tuple, direction: Tuple, count: usize) {
        let mut cyl = Cylinder::new();
        cyl.minimum = 1.0;
        cyl.maximum = 2.0;

        let r = Ray::new(point, direction.normalize());
        let xs = cyl.intersect(&r);

        assert_eq!(xs.len(), count);
    }

    #[test]
    fn intersecting_a_constrained_cylinder_scenarios() {
        intersecting_a_constrained_cylinder(
            Tuple::new_point(0.0, 1.5, 0.0),
            Tuple::new_vector(0.1, 1.0, 0.0),
            0,
        );
        intersecting_a_constrained_cylinder(
            Tuple::new_point(0.0, 3.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
            0,
        );
        intersecting_a_constrained_cylinder(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
            0,
        );
        intersecting_a_constrained_cylinder(
            Tuple::new_point(0.0, 2.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
            0,
        );
        intersecting_a_constrained_cylinder(
            Tuple::new_point(0.0, 1.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
            0,
        );
        intersecting_a_constrained_cylinder(
            Tuple::new_point(0.0, 1.5, -2.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
            2,
        );
    }

    #[test]
    fn the_default_closed_value_for_a_cylinder() {
        let cyl = Cylinder::new();

        assert!(!cyl.closed);
    }

    fn intersecting_the_caps_of_a_closed_cylinder(point: Tuple, direction: Tuple, count: usize) {
        let mut cyl = Cylinder::new();
        cyl.minimum = 1.0;
        cyl.maximum = 2.0;
        cyl.closed = true;

        let ray = Ray::new(point, direction.normalize());
        let xs = cyl.intersect(&ray);

        assert_eq!(xs.len(), count);
    }

    #[test]
    fn intersecting_the_caps_of_a_closed_cylinder_scenarios() {
        intersecting_the_caps_of_a_closed_cylinder(
            Tuple::new_point(0.0, 3.0, 0.0),
            Tuple::new_vector(0.0, -1.0, 0.0),
            2,
        );
        intersecting_the_caps_of_a_closed_cylinder(
            Tuple::new_point(0.0, 3.0, -2.0),
            Tuple::new_vector(0.0, -1.0, 2.0),
            2,
        );
        intersecting_the_caps_of_a_closed_cylinder(
            Tuple::new_point(0.0, 4.0, -2.0),
            Tuple::new_vector(0.0, -1.0, 1.0),
            2,
        );
        intersecting_the_caps_of_a_closed_cylinder(
            Tuple::new_point(0.0, 0.0, -2.0),
            Tuple::new_vector(0.0, 1.0, 2.0),
            2,
        );
        intersecting_the_caps_of_a_closed_cylinder(
            Tuple::new_point(0.0, -1.0, -2.0),
            Tuple::new_vector(0.0, 1.0, 1.0),
            2,
        );
    }
}
