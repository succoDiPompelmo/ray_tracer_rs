use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use crate::{
    intersections::Intersection, materials::Material, matrices::Matrix, rays::Ray, tuples::Tuple,
};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait Polygon {
    fn intersect(&self, original_ray: &Ray) -> Vec<f64>;
    fn normal_at(&self, point: &Tuple) -> Tuple;
}

impl Debug for dyn Polygon + Send + Sync {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Polygon Debug")
    }
}

#[derive(Clone, Debug)]
pub struct Shape {
    polygon: Arc<Mutex<dyn Polygon + Send + Sync>>,
    pub material: Material,
    transformation: Matrix,
    inverse_transformation: Option<Matrix>,
}

impl Shape {
    pub fn default(polygon: Arc<Mutex<dyn Polygon + Send + Sync>>) -> Shape {
        Shape {
            polygon,
            material: Material::default(),
            transformation: Matrix::identity(4),
            inverse_transformation: None,
        }
    }

    pub fn get_inverse_transformation(&self) -> Matrix {
        match &self.inverse_transformation {
            Some(matrix) => matrix.clone(),
            None => self.transformation.invert(),
        }
    }

    pub fn set_transformation(&mut self, trasformation: Matrix) {
        self.transformation = trasformation
    }

    pub fn precompute_inverse_transformation(&mut self) {
        self.inverse_transformation = Some(self.transformation.invert());
    }

    pub fn get_material(&self) -> &Material {
        &self.material
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let inverse_transformation = match &self.inverse_transformation {
            Some(matrix) => matrix.clone(),
            None => self.transformation.invert(),
        };
        let local_ray = ray.transform(inverse_transformation);
        let polygon = self.polygon.lock().unwrap();
        let intersections_t = polygon.intersect(&local_ray);

        let mut intersections = vec![];
        for t in intersections_t {
            intersections.push(Intersection::new(t, self.clone()))
        }

        intersections
    }

    pub fn normal_at(&self, point: &Tuple) -> Tuple {
        let inverse_transformation = match &self.inverse_transformation {
            Some(matrix) => matrix.clone(),
            None => self.transformation.invert(),
        };
        let local_point = &inverse_transformation * point;
        let polygon = self.polygon.lock().unwrap();
        let local_normal = polygon.normal_at(&local_point);
        let mut world_normal = &inverse_transformation.transpose() * &local_normal;
        world_normal.w = 0.0;

        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests {

    use std::f64::consts::PI;

    use crate::transformations::Transformation;

    use super::*;

    #[test]
    fn default_transformation() {
        let shape = Shape::default(Arc::new(Mutex::new(MockPolygon::new())));

        assert!(shape.transformation == Matrix::identity(4));
    }

    #[test]
    fn assign_transformation() {
        let mut shape = Shape::default(Arc::new(Mutex::new(MockPolygon::new())));

        shape.set_transformation(Transformation::translation(2.0, 3.0, 4.0));
        assert!(shape.transformation == Transformation::translation(2.0, 3.0, 4.0));
    }

    #[test]
    fn default_material() {
        let shape = Shape::default(Arc::new(Mutex::new(MockPolygon::new())));

        assert!(shape.get_material().clone() == Material::default());
    }

    #[test]
    fn assign_material() {
        let mut shape = Shape::default(Arc::new(Mutex::new(MockPolygon::new())));

        let mut m = Material::default();
        m.set_ambient(1.0);
        shape.set_material(m.clone());
        assert!(shape.get_material().clone() == m);
    }

    #[test]
    fn intersections_a_scaled_shape_with_a_ray() {
        let mut mock = MockPolygon::default();

        let expected_local_ray = Ray::new(
            Tuple::new_point(0.0, 0.0, -2.5),
            Tuple::new_vector(0.0, 0.0, 0.5),
        );
        mock.expect_intersect()
            .with(mockall::predicate::eq(expected_local_ray))
            .once()
            .returning(|_| vec![]);

        let mut shape = Shape::default(Arc::new(Mutex::new(mock)));
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );

        shape.set_transformation(Transformation::scaling(2.0, 2.0, 2.0));

        let xs = shape.intersect(&r);
        assert!(xs.len() == 0);
    }

    #[test]
    fn intersections_a_translated_shape_with_a_ray() {
        let mut mock = MockPolygon::default();

        let expected_local_ray = Ray::new(
            Tuple::new_point(-5.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        mock.expect_intersect()
            .with(mockall::predicate::eq(expected_local_ray))
            .once()
            .returning(|_| vec![]);

        let mut shape = Shape::default(Arc::new(Mutex::new(mock)));
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );

        shape.set_transformation(Transformation::translation(5.0, 0.0, 0.0));

        let xs = shape.intersect(&r);
        assert!(xs.len() == 0);
    }

    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let mut mock = MockPolygon::default();
        mock.expect_normal_at()
            .once()
            .returning(|p| Tuple::new_vector(p.x, p.y, p.z));

        let mut shape = Shape::default(Arc::new(Mutex::new(mock)));
        shape.set_transformation(Transformation::translation(0.0, 1.0, 0.0));

        let n = shape.normal_at(&Tuple::new_point(0.0, 1.70711, -0.70711));

        assert!(n == Tuple::new_vector(0.0, 0.7071067811865475, -0.7071067811865476));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let mut mock = MockPolygon::default();
        mock.expect_normal_at()
            .once()
            .returning(|p| Tuple::new_vector(p.x, p.y, p.z));

        let mut shape = Shape::default(Arc::new(Mutex::new(mock)));
        shape.set_transformation(
            Transformation::scaling(1.0, 0.5, 1.0) * Transformation::rotation_z(PI / 5.0),
        );

        let n = shape.normal_at(&Tuple::new_point(
            0.0,
            2.0_f64.sqrt() / 2.0,
            -2.0_f64.sqrt() / 2.0,
        ));

        assert!(n == Tuple::new_vector(0.0, 0.9701425001453319, -0.24253562503633294));
    }
}
