use float_cmp::ApproxEq;

use crate::{core::tuples::Tuple, margin::Margin, rays::Ray, shapes::Polygon};

pub struct Triangle {
    p1: Tuple,
    p2: Tuple,
    p3: Tuple,
    e1: Tuple,
    e2: Tuple,
    normal: Tuple,
}

impl Triangle {
    pub fn new(p1: Tuple, p2: Tuple, p3: Tuple) -> Triangle {
        let e1 = &p2 - &p1;
        let e2 = &p3 - &p1;
        let normal = e2.cross(&e1).normalize();

        Triangle {
            p1,
            p2,
            p3,
            e1,
            e2,
            normal,
        }
    }
}

impl Polygon for Triangle {
    fn intersect(&self, original_ray: &Ray) -> Vec<f64> {
        let dir_cross_e2 = original_ray.get_direction().cross(&self.e2);
        let det = self.e1.dot(&dir_cross_e2);

        if det.abs().approx_eq(0.0, Margin::default_f64()) {
            return vec![];
        };

        let f = 1.0 / det;
        let p1_to_origin = &original_ray.get_origin() - &self.p1;
        let u = f * p1_to_origin.dot(&dir_cross_e2);

        if !(0.0..1.0).contains(&u) {
            return vec![];
        }

        let origin_cross_e1 = p1_to_origin.cross(&self.e1);
        let v = f * original_ray.get_direction().dot(&origin_cross_e1);

        if v < 0.0 || (u + v) > 1.0 {
            return vec![];
        }

        vec![f * self.e2.dot(&origin_cross_e1)]
    }

    fn normal_at(&self, _point: &Tuple) -> Tuple {
        self.normal.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructing_a_triangle() {
        let p1 = Tuple::new_point(0.0, 1.0, 0.0);
        let p2 = Tuple::new_point(-1.0, 0.0, 0.0);
        let p3 = Tuple::new_point(1.0, 0.0, 0.0);

        let t = Triangle::new(p1.clone(), p2.clone(), p3.clone());

        assert_eq!(t.p1, p1);
        assert_eq!(t.p2, p2);
        assert_eq!(t.p3, p3);
        assert_eq!(t.e1, Tuple::new_vector(-1.0, -1.0, 0.0));
        assert_eq!(t.e2, Tuple::new_vector(1.0, -1.0, 0.0));
        assert_eq!(t.normal, Tuple::new_vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn finding_the_normal_on_a_triangle() {
        let t = Triangle::new(
            Tuple::new_point(0.0, 1.0, 0.0),
            Tuple::new_point(-1.0, 0.0, 0.0),
            Tuple::new_point(1.0, 0.0, 0.0),
        );
        let n1 = t.normal_at(&Tuple::new_point(0.0, 0.5, 0.0));
        let n2 = t.normal_at(&Tuple::new_point(-0.5, 0.75, 0.0));
        let n3 = t.normal_at(&Tuple::new_point(0.5, 0.25, 0.0));

        assert_eq!(n1, t.normal);
        assert_eq!(n2, t.normal);
        assert_eq!(n3, t.normal);
    }

    #[test]
    fn intersecting_a_ray_parallel_to_the_triangle() {
        let t = Triangle::new(
            Tuple::new_point(0.0, 1.0, 0.0),
            Tuple::new_point(-1.0, 0.0, 0.0),
            Tuple::new_point(1.0, 0.0, 0.0),
        );
        let r = Ray::new(
            Tuple::new_point(0.0, -1.0, -2.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );

        let xs = t.intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_misses_the_p1_p3_edge() {
        let t = Triangle::new(
            Tuple::new_point(0.0, 1.0, 0.0),
            Tuple::new_point(-1.0, 0.0, 0.0),
            Tuple::new_point(1.0, 0.0, 0.0),
        );
        let r = Ray::new(
            Tuple::new_point(1.0, 1.0, -2.0),
            Tuple::new_point(0.0, 0.0, 1.0),
        );

        let xs = t.intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_misses_the_p1_p2_edge() {
        let t = Triangle::new(
            Tuple::new_point(0.0, 1.0, 0.0),
            Tuple::new_point(-1.0, 0.0, 0.0),
            Tuple::new_point(1.0, 0.0, 0.0),
        );
        let r = Ray::new(
            Tuple::new_point(-1.0, 1.0, -2.0),
            Tuple::new_point(0.0, 0.0, 1.0),
        );

        let xs = t.intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_misses_the_p2_p3_edge() {
        let t = Triangle::new(
            Tuple::new_point(0.0, 1.0, 0.0),
            Tuple::new_point(-1.0, 0.0, 0.0),
            Tuple::new_point(1.0, 0.0, 0.0),
        );
        let r = Ray::new(
            Tuple::new_point(0.0, -1.0, -2.0),
            Tuple::new_point(0.0, 0.0, 1.0),
        );

        let xs = t.intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_strikes_a_triangle() {
        let t = Triangle::new(
            Tuple::new_point(0.0, 1.0, 0.0),
            Tuple::new_point(-1.0, 0.0, 0.0),
            Tuple::new_point(1.0, 0.0, 0.0),
        );
        let r = Ray::new(
            Tuple::new_point(0.0, 0.5, -2.0),
            Tuple::new_point(0.0, 0.0, 1.0),
        );

        let xs = t.intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0], 2.0);
    }
}
