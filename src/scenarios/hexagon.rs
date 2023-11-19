use std::{sync::{Arc, Mutex}, f64::consts::PI};

use crate::{groups::Group, world::World, shapes::Shape, transformations::Transformation, spheres::Sphere, cylinders::Cylinder};

use super::Scenario;

const NAME: &str = "Hexagon";

pub struct Hexagon {}

impl Hexagon {
    pub fn new() -> Scenario {
        let mut hex = Group::new();
        let parent_id = 0;

        for n in 0..=5 {
            hexagon_side(&mut hex, parent_id, n);
        }

        let mut world = World::new();
        world.add_group(hex);

        Scenario {
            name: NAME.to_owned(),
            world,
        }
    }

    pub fn name() -> String {
        NAME.to_owned()
    }
}

fn hexagon_corner(parent_id: usize) -> Shape {
    let mut corner = Shape::default(Arc::new(Mutex::new(Sphere::new())));
    corner.set_transformation(
        Transformation::translation(0.0, 0.0, -1.0) * Transformation::scaling(0.25, 0.25, 0.25),
    );
    corner.precompute_inverse_transformation();
    corner.set_parent_id(parent_id);

    corner
}

fn hexagon_edge(parent_id: usize) -> Shape {
    let mut cylinder = Cylinder::new();
    cylinder.set_minimum(0.0);
    cylinder.set_maximum(1.0);
    let mut edge = Shape::default(Arc::new(Mutex::new(cylinder)));
    edge.set_transformation(
        Transformation::translation(0.0, 0.0, -1.0)
            * Transformation::rotation_y(-PI / 6.0)
            * Transformation::rotation_z(-PI / 2.0)
            * Transformation::scaling(0.25, 1.00, 0.25),
    );
    edge.precompute_inverse_transformation();
    edge.set_parent_id(parent_id);

    edge
}

fn hexagon_side(g: &mut Group, parent_id: usize, n: usize) {
    let rotation = Transformation::rotation_y(n as f64 * (PI / 3.0));
    let matrix_id = g.add_matrix(rotation, Some(parent_id));
    g.add_node(hexagon_corner(matrix_id), Some(matrix_id));
    g.add_node(hexagon_edge(matrix_id), Some(matrix_id));
}