use r3bl_rs_utils::Arena;

use crate::{intersections::Intersection, matrices::Matrix, rays::Ray, shapes::Shape};

#[derive(Debug)]
pub struct Group {
    pub arena: Arena<NodeTypes>,
}

#[derive(Clone, Debug)]
pub enum NodeTypes {
    Shape(Box<Shape>),
    Matrix(Matrix),
}

impl Group {
    pub fn new() -> Group {
        let mut arena = Arena::<NodeTypes>::new();
        let root_id = arena.add_new_node(NodeTypes::Matrix(Matrix::identity(4)), None);

        println!("Root Node ID: {:?}", root_id);

        Group { arena }
    }

    pub fn add_matrix(&mut self, matrix: Matrix, parent_id: Option<usize>) -> usize {
        self.arena
            .add_new_node(NodeTypes::Matrix(matrix), parent_id)
    }

    pub fn add_node(&mut self, shape: Shape, parent_id: Option<usize>) -> usize {
        self.arena
            .add_new_node(NodeTypes::Shape(Box::new(shape)), parent_id)
    }

    pub fn intersect(&mut self, original_ray: &Ray, node_id: usize) -> Vec<Intersection> {
        let mut xs = vec![];

        let maybe_childs = self.arena.get_children_of(node_id);

        if let Some(childs_id) = maybe_childs {
            for child_id in childs_id {
                let mut i = match self.arena.get_node_arc(child_id) {
                    None => return vec![],
                    Some(a) => {
                        let payload = a.read().unwrap();
                        match &payload.payload {
                            NodeTypes::Matrix(matrix) => {
                                let local_ray = original_ray.transform(matrix.invert());
                                self.intersect(&local_ray, payload.id)
                            }
                            NodeTypes::Shape(shape) => shape.intersect(original_ray),
                        }
                    }
                };

                xs.append(&mut i)
            }
        };

        xs
    }
}

#[cfg(test)]
mod tests {

    use std::sync::{Arc, Mutex};

    use crate::{spheres::Sphere, transformations::Transformation, tuples::Tuple};

    use super::*;

    #[test]
    fn intersecting_a_ray_with_an_empty_group() {
        let mut g = Group::new();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );

        let xs = g.intersect(&r, 0);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersecting_a_ray_with_a_nonempty_group() {
        let mut g = Group::new();
        let s1 = Shape::default(Arc::new(Mutex::new(Sphere::new())));
        let mut s2 = Shape::default(Arc::new(Mutex::new(Sphere::new())));
        let mut s3 = Shape::default(Arc::new(Mutex::new(Sphere::new())));

        s2.set_transformation(Transformation::translation(0.0, 0.0, -3.0));
        s3.set_transformation(Transformation::translation(5.0, 0.0, 0.0));

        g.add_node(s1, Some(0));
        g.add_node(s2, Some(0));
        g.add_node(s3, Some(0));

        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let xs = g.intersect(&r, 0);

        println!("{:?}", xs);

        assert_eq!(xs.len(), 4);
    }

    #[test]
    fn intersecting_a_transformed_group() {
        let mut g = Group::new();
        let matrix_id = g.add_matrix(Transformation::scaling(2.0, 2.0, 2.0), Some(0));

        let mut s = Shape::default(Arc::new(Mutex::new(Sphere::new())));
        s.set_transformation(Transformation::translation(5.0, 0.0, 0.0));
        g.add_node(s, Some(matrix_id));

        let r = Ray::new(
            Tuple::new_point(10.0, 0.0, -10.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let xs = g.intersect(&r, 0);

        println!("{:?}", xs);

        assert_eq!(xs.len(), 2);
    }
}
