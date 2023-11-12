use crate::{groups::Group, intersections::Intersection, rays::Ray, shapes::Shape};

#[derive(Debug)]
pub enum Objects {
    Shape(Box<Shape>),
    Group(Group),
}

impl Objects {
    pub fn intersect(&mut self, ray: &Ray) -> Vec<Intersection> {
        match self {
            Objects::Group(g) => g.intersect(ray, 0),
            Objects::Shape(s) => s.intersect(ray),
        }
    }
}
