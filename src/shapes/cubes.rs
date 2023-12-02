use float_cmp::ApproxEq;

use crate::{margin::Margin, rays::Ray, shapes::Polygon, core::tuples::Tuple};

pub struct Cube {}

impl Cube {
    pub fn new() -> Cube {
        Cube {}
    }
}

impl Polygon for Cube {
    fn intersect(&self, original_ray: &Ray) -> Vec<f64> {
        let (xtmin, xtmax) =
            check_axis(original_ray.get_origin().x, original_ray.get_direction().x);
        let (ytmin, ytmax) =
            check_axis(original_ray.get_origin().y, original_ray.get_direction().y);
        let (ztmin, ztmax) =
            check_axis(original_ray.get_origin().z, original_ray.get_direction().z);

        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);

        if tmin > tmax {
            return vec![];
        }

        vec![tmin, tmax]
    }

    fn normal_at(&self, point: &Tuple) -> Tuple {
        let maxc = point.x.abs().max(point.y.abs()).max(point.z.abs());

        if maxc.approx_eq(point.x.abs(), Margin::default_f64()) {
            return Tuple::new_vector(point.x, 0.0, 0.0);
        }

        if maxc.approx_eq(point.y.abs(), Margin::default_f64()) {
            return Tuple::new_vector(0.0, point.y, 0.0);
        }

        Tuple::new_vector(0.0, 0.0, point.z)
    }
}

fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;

    let (tmin, tmax) = if direction.abs() > 0.0000001 {
        (tmin_numerator / direction, tmax_numerator / direction)
    } else {
        (
            tmin_numerator * 1_000_000_000_000_000.0,
            tmax_numerator * 1_000_000_000_000_000.0,
        )
    };

    if tmin > tmax {
        return (tmax, tmin);
    }

    (tmin, tmax)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn a_ray_intersects_a_cube(point: Tuple, direction: Tuple, t1: f64, t2: f64) {
        let c = Cube::new();
        let r = Ray::new(point, direction);

        let xs = c.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(*xs.get(0).unwrap(), t1);
        assert_eq!(*xs.get(1).unwrap(), t2);
    }

    #[test]
    fn a_ray_intersects_a_cube_positive_x() {
        a_ray_intersects_a_cube(
            Tuple::new_point(5.0, 0.5, 0.0),
            Tuple::new_vector(-1.0, 0.0, 0.0),
            4.0,
            6.0,
        );
    }

    #[test]
    fn a_ray_intersects_a_cube_negative_x() {
        a_ray_intersects_a_cube(
            Tuple::new_point(-5.0, 0.5, 0.0),
            Tuple::new_vector(1.0, 0.0, 0.0),
            4.0,
            6.0,
        );
    }

    #[test]
    fn a_ray_intersects_a_cube_positive_y() {
        a_ray_intersects_a_cube(
            Tuple::new_point(0.5, 5.0, 0.0),
            Tuple::new_vector(0.0, -1.0, 0.0),
            4.0,
            6.0,
        );
    }

    #[test]
    fn a_ray_intersects_a_cube_negative_y() {
        a_ray_intersects_a_cube(
            Tuple::new_point(0.5, -5.0, 0.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
            4.0,
            6.0,
        );
    }

    #[test]
    fn a_ray_intersects_a_cube_positive_z() {
        a_ray_intersects_a_cube(
            Tuple::new_point(0.5, 0.0, 5.0),
            Tuple::new_vector(0.0, 0.0, -1.0),
            4.0,
            6.0,
        );
    }

    #[test]
    fn a_ray_intersects_a_cube_negative_z() {
        a_ray_intersects_a_cube(
            Tuple::new_point(0.5, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
            4.0,
            6.0,
        );
    }

    #[test]
    fn a_ray_intersects_a_cube_inside() {
        a_ray_intersects_a_cube(
            Tuple::new_point(0.0, 0.5, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
            -1.0,
            1.0,
        );
    }

    fn a_ray_misses_a_cube(point: Tuple, direction: Tuple) {
        let c = Cube::new();
        let r = Ray::new(point, direction);

        let xs = c.intersect(&r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_misses_a_cube_scenarios() {
        a_ray_misses_a_cube(
            Tuple::new_point(-2.0, 0.0, 0.0),
            Tuple::new_vector(0.2673, 0.5345, 0.8018),
        );
        a_ray_misses_a_cube(
            Tuple::new_point(0.0, -2.0, 0.0),
            Tuple::new_vector(0.8018, 0.2673, 0.5345),
        );
        a_ray_misses_a_cube(
            Tuple::new_point(0.0, 0.0, -2.0),
            Tuple::new_vector(0.5345, 0.8018, 0.2673),
        );
        a_ray_misses_a_cube(
            Tuple::new_point(2.0, 0.0, 2.0),
            Tuple::new_vector(0.0, 0.0, -1.0),
        );
        a_ray_misses_a_cube(
            Tuple::new_point(0.0, 2.0, 2.0),
            Tuple::new_vector(0.0, -1.0, 0.0),
        );
        a_ray_misses_a_cube(
            Tuple::new_point(2.0, 2.0, 0.0),
            Tuple::new_vector(-1.0, 0.0, 0.0),
        );
    }

    fn the_normal_on_the_surface_of_a_cube(point: Tuple, normal: Tuple) {
        let c = Cube::new();
        assert_eq!(normal, c.normal_at(&point));
    }

    #[test]
    fn the_normal_on_the_surface_of_a_cube_scenarios() {
        the_normal_on_the_surface_of_a_cube(
            Tuple::new_point(1.0, 0.5, -0.8),
            Tuple::new_vector(1.0, 0.0, 0.0),
        );
        the_normal_on_the_surface_of_a_cube(
            Tuple::new_point(-1.0, -0.2, 0.9),
            Tuple::new_vector(-1.0, 0.0, 0.0),
        );
        the_normal_on_the_surface_of_a_cube(
            Tuple::new_point(-0.4, 1.0, -0.1),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );
        the_normal_on_the_surface_of_a_cube(
            Tuple::new_point(0.3, -1.0, -0.7),
            Tuple::new_vector(0.0, -1.0, 0.0),
        );
        the_normal_on_the_surface_of_a_cube(
            Tuple::new_point(-0.6, 0.3, 1.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        the_normal_on_the_surface_of_a_cube(
            Tuple::new_point(0.4, 0.4, -1.0),
            Tuple::new_vector(0.0, 0.0, -1.0),
        );
        the_normal_on_the_surface_of_a_cube(
            Tuple::new_point(1.0, 1.0, 1.0),
            Tuple::new_vector(1.0, 0.0, 0.0),
        );
        the_normal_on_the_surface_of_a_cube(
            Tuple::new_point(-1.0, -1.0, -1.0),
            Tuple::new_vector(-1.0, 0.0, 0.0),
        );
    }
}
