use crate::{rays::Ray, shapes::Polygon, tuples::Tuple};

pub struct Plane {}

impl Plane {
    pub fn new() -> Plane {
        Plane {}
    }
}

impl Polygon for Plane {
    fn intersect(&self, original_ray: &Ray) -> Vec<f64> {
        if original_ray.get_direction().y.abs() < 0.000001 {
            return vec![];
        }

        let t = -original_ray.get_origin().y / original_ray.get_direction().y;
        vec![t]
    }

    fn normal_at(&self, _point: &crate::tuples::Tuple) -> Tuple {
        Tuple::new_vector(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {

    use float_cmp::ApproxEq;

    use crate::{margin::Margin, rays::Ray, tuples::Tuple};

    use super::*;

    #[test]
    fn the_normarl_of_a_plane_is_constant_everywhere() {
        let p = Plane::new();
        let n1 = p.normal_at(&Tuple::new_point(0.0, 0.0, 0.0));
        let n2 = p.normal_at(&Tuple::new_point(10.0, 0.0, -10.0));
        let n3 = p.normal_at(&Tuple::new_point(-5.0, 0.0, 150.0));

        let n = Tuple::new_vector(0.0, 1.0, 0.0);

        assert!(n1 == n);
        assert!(n2 == n);
        assert!(n3 == n);
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = Plane::new();
        let r = Ray::new(
            Tuple::new_point(0.0, 10.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let xs = p.intersect(&r);

        assert!(xs.is_empty())
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p = Plane::new();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let xs = p.intersect(&r);

        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = Plane::new();
        let r = Ray::new(
            Tuple::new_point(0.0, 1.0, 0.0),
            Tuple::new_vector(0.0, -1.0, 0.0),
        );
        let xs = p.intersect(&r);

        assert!(xs.len() == 1);
        assert!(xs.get(0).unwrap().approx_eq(1.0, Margin::default_f64()));
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Plane::new();
        let r = Ray::new(
            Tuple::new_point(0.0, -1.0, 0.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );
        let xs = p.intersect(&r);

        assert!(xs.len() == 1);
        assert!(xs.get(0).unwrap().approx_eq(1.0, Margin::default_f64()));
    }
}
