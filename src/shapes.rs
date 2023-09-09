use crate::{
    intersections::Intersection, materials::Material, matrices::Matrix, rays::Ray, spheres::Sphere,
    tuples::Tuple,
};

#[cfg(test)]
use mockall::{automock, mock, predicate::*};

#[cfg_attr(test, automock)]
pub trait Polygon {
    fn intersect(&self, original_ray: &Ray) -> Vec<Intersection>;
}

pub struct Shape {
    polygon: Box<dyn Polygon>,
    material: Material,
    transformation: Matrix,
}

impl Shape {
    fn default(polygon: Box<dyn Polygon>) -> Shape {
        Shape {
            polygon,
            material: Material::default(),
            transformation: Matrix::identity(4),
        }
    }

    fn get_transformation(&self) -> &Matrix {
        &self.transformation
    }

    fn set_transformation(&mut self, trasformation: Matrix) {
        self.transformation = trasformation
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn set_material(&mut self, material: Material) {
        self.material = material
    }

    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let local_ray = ray.transform(self.transformation.invert());
        self.polygon.intersect(&local_ray)
    }
}

#[cfg(test)]
mod tests {

    use crate::transformations::Transformation;

    use super::*;

    #[test]
    fn default_transformation() {
        let shape = Shape::default(Box::new(MockPolygon::new()));

        assert!(shape.get_transformation().clone() == Matrix::identity(4));
    }

    #[test]
    fn assign_transformation() {
        let mut shape = Shape::default(Box::new(MockPolygon::new()));

        shape.set_transformation(Transformation::translation(2.0, 3.0, 4.0));
        assert!(shape.get_transformation().clone() == Transformation::translation(2.0, 3.0, 4.0));
    }

    #[test]
    fn default_material() {
        let shape = Shape::default(Box::new(MockPolygon::new()));

        assert!(shape.get_material().clone() == Material::default());
    }

    #[test]
    fn assign_material() {
        let mut shape = Shape::default(Box::new(MockPolygon::new()));

        let mut m = Material::default();
        m.set_ambient(1.0);
        shape.set_material(m.clone());
        assert!(shape.get_material().clone() == m);
    }

    #[test]
    fn intersections_a_scaled_shape_with_a_ray() {
        let mut mock = Box::new(MockPolygon::default());

        let expected_local_ray = Ray::new(
            Tuple::new_point(0.0, 0.0, -2.5),
            Tuple::new_vector(0.0, 0.0, 0.5),
        );
        mock.expect_intersect()
            .with(mockall::predicate::eq(expected_local_ray))
            .once()
            .returning(|_| vec![]);

        let mut shape = Shape::default(mock);
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
        let mut mock = Box::new(MockPolygon::default());

        let expected_local_ray = Ray::new(
            Tuple::new_point(-5.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        mock.expect_intersect()
            .with(mockall::predicate::eq(expected_local_ray))
            .once()
            .returning(|_| vec![]);

        let mut shape = Shape::default(mock);
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );

        shape.set_transformation(Transformation::translation(5.0, 0.0, 0.0));

        let xs = shape.intersect(&r);
        assert!(xs.len() == 0);
    }
}
