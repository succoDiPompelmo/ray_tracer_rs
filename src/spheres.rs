use float_cmp::{ApproxEq, F64Margin};

use crate::{
    intersections::Intersection, materials::Material, matrices::Matrix, rays::Ray, tuples::Tuple,
};

#[derive(Clone, Debug)]
pub struct Sphere {
    pub material: Material,
    center: Tuple,
    radius: f64,
    transform: Matrix,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            material: Material::default(),
            center: Tuple::new_point(0.0, 0.0, 0.0),
            radius: 1.0,
            transform: Matrix::identity(4),
        }
    }

    pub fn intersect(&self, original_ray: &Ray) -> Vec<Intersection> {
        let ray = original_ray.transform(self.get_transform().invert());
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

        Intersection::intersects(&[
            Intersection::new(t1, self.clone()),
            Intersection::new(t2, self.clone()),
        ])
    }

    fn get_transform(&self) -> Matrix {
        self.transform.clone()
    }

    pub fn set_transformation(&mut self, t: Matrix) {
        self.transform = t
    }

    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_point = &self.transform.invert() * &world_point;
        let object_normal = object_point - self.center;
        let mut world_normal = &self.transform.invert().transpose() * &object_normal;

        world_normal.w = 0.0;

        return world_normal.normalize();
    }

    pub fn get_material(&self) -> Material {
        self.material.clone()
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
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

    use std::f64::consts::PI;

    use crate::{rays::Ray, transformations::Transformation, tuples::Tuple};

    use super::*;

    #[test]
    fn ray_intersect_spheres_in_two_points() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();

        let xs = s.intersect(&r);

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

        let xs = s.intersect(&r);

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

        let xs = s.intersect(&r);

        assert!(xs.len() == 0);
    }

    #[test]
    fn ray_originates_inside_the_sphere() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();

        let xs = s.intersect(&r);

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

        let xs = s.intersect(&r);

        assert!(xs.len() == 2);
        assert!(xs.get(0).unwrap().get_object() == s);
        assert!(xs.get(1).unwrap().get_object() == s);
        assert!(xs.get(0).unwrap().get_t() == -6.0);
        assert!(xs.get(1).unwrap().get_t() == -4.0);
    }

    #[test]
    fn sphere_default_transformation() {
        let s = Sphere::new();

        assert!(s.get_transform() == Matrix::identity(4))
    }

    #[test]
    fn change_sphere_transformation() {
        let mut s = Sphere::new();
        let t = Transformation::translation(2.0, 3.0, 4.0);
        s.set_transformation(t.clone());

        assert!(s.get_transform() == t);
    }

    #[test]
    fn intesecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let mut s = Sphere::new();
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
        let mut s = Sphere::new();
        let t = Transformation::translation(5.0, 0.0, 0.0);
        s.set_transformation(t.clone());

        let xs = s.intersect(&r);

        assert!(xs.len() == 0);
    }

    #[test]
    fn normal_on_a_sphere() {
        let s = Sphere::new();

        let v1 = Tuple::new_vector(1.0, 0.0, 0.0);
        assert!(s.normal_at(Tuple::new_point(1.0, 0.0, 0.0)) == v1);

        let v2 = Tuple::new_vector(0.0, 1.0, 0.0);
        assert!(s.normal_at(Tuple::new_point(0.0, 1.0, 0.0)) == v2);

        let v3 = Tuple::new_vector(0.0, 0.0, 1.0);
        assert!(s.normal_at(Tuple::new_point(0.0, 0.0, 1.0)) == v3);

        let value = 3.0_f64.sqrt() / 3.0;
        let v4 = Tuple::new_vector(value, value, value);
        assert!(s.normal_at(Tuple::new_point(value, value, value)) == v4);
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = Sphere::new();

        let value = 3.0_f64.sqrt() / 3.0;
        let n = s.normal_at(Tuple::new_point(value, value, value));
        assert!(n.normalize() == n);
    }

    #[test]
    fn normal_on_a_translated_sphere() {
        let mut s = Sphere::new();
        s.set_transformation(Transformation::translation(0.0, 1.0, 0.0));
        let n = s.normal_at(Tuple::new_point(0.0, 1.70711, -0.70711));

        println!("{:?}", n);

        assert!(n == Tuple::new_vector(0.0, 0.7071067811865475, -0.7071067811865476))
    }

    #[test]
    fn normal_on_a_transformed_sphere() {
        let mut s = Sphere::new();
        s.set_transformation(
            Transformation::scaling(1.0, 0.5, 1.0) * Transformation::rotation_z(PI / 5.0),
        );
        let n = s.normal_at(Tuple::new_point(
            0.0,
            2.0_f64.sqrt() / 2.0,
            -2.0_f64.sqrt() / 2.0,
        ));

        assert!(n == Tuple::new_vector(0.0, 0.9701425001453319, -0.24253562503633294))
    }

    #[test]
    fn a_sphere_has_a_default_material() {
        let s = Sphere::new();

        assert!(s.material == Material::default());
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let mut s = Sphere::new();
        let mut m = Material::default();

        m.set_ambient(1.0);
        s.set_material(m.clone());

        assert!(s.material == m);
    }
}
