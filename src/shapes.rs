use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use crate::{
    groups::{Group, NodeTypes},
    intersections::Intersection,
    materials::Material,
    matrices::Matrix,
    rays::Ray,
    tuples::Tuple,
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
    id: Option<usize>,
    parent_id: Option<usize>,
    polygon: Arc<Mutex<dyn Polygon + Send + Sync>>,
    pub material: Material,
    transformation: Matrix,
    inverse_transformation: Option<Matrix>,
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.polygon, &other.polygon)
    }
}

impl Shape {
    pub fn default(polygon: Arc<Mutex<dyn Polygon + Send + Sync>>) -> Shape {
        Shape {
            id: None,
            parent_id: None,
            polygon,
            material: Material::default(),
            transformation: Matrix::identity(4),
            inverse_transformation: None,
        }
    }

    #[cfg(test)]
    pub fn glass(polygon: Arc<Mutex<dyn Polygon + Send + Sync>>) -> Shape {
        let mut material = Material::default();
        material.set_transparency(1.0);
        material.set_refractive_index(1.5);

        Shape {
            id: None,
            parent_id: None,
            polygon,
            material,
            transformation: Matrix::identity(4),
            inverse_transformation: None,
        }
    }

    pub fn get_id(&self) -> Option<usize> {
        self.id
    }

    pub fn get_parent_id(&self) -> Option<usize> {
        self.parent_id
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = Some(id)
    }

    pub fn set_parent_id(&mut self, id: usize) {
        self.parent_id = Some(id)
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

    pub fn normal_at(&self, point: &Tuple, g: Option<&Group>) -> Tuple {
        let local_point = self.world_to_object(point, g);
        let polygon = self.polygon.lock().unwrap();
        let local_normal = polygon.normal_at(&local_point);
        self.normal_to_world(&local_normal, g)
    }

    fn world_to_object(&self, world_point: &Tuple, g: Option<&Group>) -> Tuple {
        let inverse_transformation = match &self.inverse_transformation {
            Some(matrix) => matrix.clone(),
            None => self.transformation.invert(),
        };

        if g.is_none() {
            return &inverse_transformation * world_point;
        }

        let mut object_point = world_point.clone();
        let mut parent_id = self.parent_id;

        let mut matrices_chain = vec![inverse_transformation];

        while parent_id.is_some() {
            let a = g.unwrap().arena.get_node_arc(parent_id.unwrap()).unwrap();
            let b = &a.read().unwrap().payload;

            let parent_matrix = match b {
                NodeTypes::Matrix(matrix) => matrix.invert(),
                NodeTypes::Shape(shape) => shape.get_inverse_transformation(),
            };
            matrices_chain.push(parent_matrix);

            parent_id = g.unwrap().arena.get_parent_of(parent_id.unwrap());
        }

        for matrix in matrices_chain.iter().rev() {
            object_point = matrix * &object_point;
        }

        object_point
    }

    fn normal_to_world(&self, object_normal: &Tuple, g: Option<&Group>) -> Tuple {
        let inverse_transformation = match &self.inverse_transformation {
            Some(matrix) => matrix.clone(),
            None => self.transformation.invert(),
        };

        if g.is_none() {
            let mut world_normal = &inverse_transformation.transpose() * object_normal;
            world_normal.w = 0.0;

            return world_normal.normalize();
        }

        let mut world_normal = object_normal.clone();
        let mut parent_id = self.parent_id;

        let mut matrices_chain = vec![inverse_transformation];

        while parent_id.is_some() {
            let a = g.unwrap().arena.get_node_arc(parent_id.unwrap()).unwrap();
            let b = &a.read().unwrap().payload;

            let parent_matrix = match b {
                NodeTypes::Matrix(matrix) => matrix.invert(),
                NodeTypes::Shape(shape) => shape.get_inverse_transformation(),
            };
            matrices_chain.push(parent_matrix);

            parent_id = g.unwrap().arena.get_parent_of(parent_id.unwrap());
        }

        for matrix in matrices_chain.iter() {
            world_normal = &matrix.transpose() * &world_normal;
            world_normal.w = 0.0;
            world_normal = world_normal.normalize();
        }

        world_normal
    }
}

#[cfg(test)]
mod tests {

    use std::f64::consts::PI;

    use float_cmp::ApproxEq;

    use crate::{
        groups::{Group, NodeTypes},
        margin::Margin,
        spheres::Sphere,
        transformations::Transformation,
    };

    use super::*;

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

        let n = shape.normal_at(&Tuple::new_point(0.0, 1.70711, -0.70711), None);

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

        let n = shape.normal_at(
            &Tuple::new_point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0),
            None,
        );

        assert!(n == Tuple::new_vector(0.0, 0.9701425001453319, -0.24253562503633294));
    }

    #[test]
    fn a_helper_for_producing_a_shape_with_a_glassy_material() {
        let mock = MockPolygon::default();
        let shape = Shape::glass(Arc::new(Mutex::new(mock)));

        assert!(shape.transformation == Matrix::identity(4));
        assert!(shape
            .material
            .get_transparency()
            .approx_eq(1.0, Margin::default_f64()));
        assert!(shape
            .material
            .get_refractive_index()
            .approx_eq(1.5, Margin::default_f64()));
    }

    fn n1_n2_scenario() -> (Ray, Vec<Intersection>) {
        let mut a = Shape::glass(Arc::new(Mutex::new(Sphere::new())));

        let a_transform = Transformation::translation(2.0, 2.0, 2.0);
        let mut a_material = Material::default();
        a_material.set_transparency(1.0);
        a_material.set_refractive_index(1.5);
        a.set_transformation(a_transform);
        a.set_material(a_material);

        let mut b = Shape::glass(Arc::new(Mutex::new(Sphere::new())));

        let b_transform = Transformation::translation(0.0, 0.0, -0.25);
        let mut b_material = Material::default();
        b_material.set_transparency(1.0);
        b_material.set_refractive_index(2.0);
        b.set_transformation(b_transform);
        b.set_material(b_material);

        let mut c = Shape::glass(Arc::new(Mutex::new(Sphere::new())));

        let c_transform = Transformation::translation(0.0, 0.0, 0.25);
        let mut c_material = Material::default();
        c_material.set_transparency(1.0);
        c_material.set_refractive_index(2.5);
        c.set_transformation(c_transform);
        c.set_material(c_material);

        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -4.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );

        let xs = Intersection::intersects(&[
            Intersection::new(2.0, a.clone()),
            Intersection::new(2.75, b.clone()),
            Intersection::new(3.25, c.clone()),
            Intersection::new(4.75, b.clone()),
            Intersection::new(5.25, c.clone()),
            Intersection::new(6.0, a.clone()),
        ]);

        (r, xs)
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections_0() {
        let (r, xs) = n1_n2_scenario();
        let comps = xs.get(0).unwrap().prepare_computations(&r, &xs);

        assert!(comps.get_n1().approx_eq(1.0, Margin::default_f64()));
        assert!(comps.get_n2().approx_eq(1.5, Margin::default_f64()));
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections_1() {
        let (r, xs) = n1_n2_scenario();
        let comps = xs.get(1).unwrap().prepare_computations(&r, &xs);

        assert!(comps.get_n1().approx_eq(1.5, Margin::default_f64()));
        assert!(comps.get_n2().approx_eq(2.0, Margin::default_f64()));
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections_2() {
        let (r, xs) = n1_n2_scenario();
        let comps = xs.get(2).unwrap().prepare_computations(&r, &xs);

        assert!(comps.get_n1().approx_eq(2.0, Margin::default_f64()));
        assert!(comps.get_n2().approx_eq(2.5, Margin::default_f64()));
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections_3() {
        let (r, xs) = n1_n2_scenario();
        let comps = xs.get(3).unwrap().prepare_computations(&r, &xs);

        assert!(comps.get_n1().approx_eq(2.5, Margin::default_f64()));
        assert!(comps.get_n2().approx_eq(2.5, Margin::default_f64()));
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections_4() {
        let (r, xs) = n1_n2_scenario();
        let comps = xs.get(4).unwrap().prepare_computations(&r, &xs);

        assert!(comps.get_n1().approx_eq(2.5, Margin::default_f64()));
        assert!(comps.get_n2().approx_eq(1.5, Margin::default_f64()));
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections_5() {
        let (r, xs) = n1_n2_scenario();
        let comps = xs.get(5).unwrap().prepare_computations(&r, &xs);

        assert!(comps.get_n1().approx_eq(1.5, Margin::default_f64()));
        assert!(comps.get_n2().approx_eq(1.0, Margin::default_f64()));
    }

    #[test]
    fn converting_a_point_from_world_to_object_space() {
        let mut g = Group::new();
        let m1 = Transformation::rotation_y(PI / 2.0);
        let m2 = Transformation::scaling(2.0, 2.0, 2.0);
        let mut s = Shape::default(Arc::new(Mutex::new(Sphere::new())));
        s.set_transformation(Transformation::translation(5.0, 0.0, 0.0));

        let m1_id = g.add_matrix(m1, None);
        let m2_id = g.add_matrix(m2, Some(m1_id));
        s.set_parent_id(m2_id);
        let s_id = g.add_node(s, Some(m2_id));

        let a = g.arena.get_node_arc(s_id).unwrap();
        let b = &a.read().unwrap().payload;

        let shape = match b {
            NodeTypes::Shape(shape) => shape,
            NodeTypes::Matrix(_) => panic!(),
        };

        let p = shape.world_to_object(&Tuple::new_point(-2.0, 0.0, -10.0), Some(&g));
        assert_eq!(p, Tuple::new_point(0.0, 0.0, -1.0));
    }

    #[test]
    fn converting_a_normal_from_object_to_world_space() {
        let mut g = Group::new();
        let m1 = Transformation::rotation_y(PI / 2.0);
        let m2 = Transformation::scaling(1.0, 2.0, 3.0);
        let mut s = Shape::default(Arc::new(Mutex::new(Sphere::new())));
        s.set_transformation(Transformation::translation(5.0, 0.0, 0.0));

        let m1_id = g.add_matrix(m1, None);
        let m2_id = g.add_matrix(m2, Some(m1_id));
        s.set_parent_id(m2_id);
        let s_id = g.add_node(s, Some(m2_id));

        let a = g.arena.get_node_arc(s_id).unwrap();
        let b = &a.read().unwrap().payload;

        let shape = match b {
            NodeTypes::Shape(shape) => shape,
            NodeTypes::Matrix(_) => panic!(),
        };

        let p = shape.normal_to_world(
            &Tuple::new_vector(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
            ),
            Some(&g),
        );
        assert_eq!(
            p,
            Tuple::new_vector(
                0.28571428571428575,
                0.42857142857142855,
                -0.8571428571428571
            )
        );
    }

    #[test]
    fn finding_the_normal_on_a_child_object() {
        let mut g = Group::new();
        let m1 = Transformation::rotation_y(PI / 2.0);
        let m2 = Transformation::scaling(1.0, 2.0, 3.0);
        let mut s = Shape::default(Arc::new(Mutex::new(Sphere::new())));
        s.set_transformation(Transformation::translation(5.0, 0.0, 0.0));

        let m1_id = g.add_matrix(m1, None);
        let m2_id = g.add_matrix(m2, Some(m1_id));
        s.set_parent_id(m2_id);
        let s_id = g.add_node(s, Some(m2_id));

        let a = g.arena.get_node_arc(s_id).unwrap();
        let b = &a.read().unwrap().payload;

        let shape = match b {
            NodeTypes::Shape(shape) => shape,
            NodeTypes::Matrix(_) => panic!(),
        };

        let p = shape.normal_at(&Tuple::new_point(1.7321, 1.1547, -5.5774), Some(&g));
        assert_eq!(
            p,
            Tuple::new_vector(
                0.28570368184140726,
                0.42854315178114105,
                -0.8571605294481017
            )
        );
    }
}
