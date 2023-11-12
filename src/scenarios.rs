use std::{sync::{Arc, Mutex}, f64::consts::PI};

use crate::{tuples::Tuple, groups::Group, world::World, materials::Material, transformations::Transformation, shapes::Shape, spheres::Sphere, patterns::{Pattern, PatternsKind}, planes::Plane, cylinders::Cylinder};

pub struct Scenario {}

impl Scenario {
    pub fn hexagon() -> World {
        let mut hex = Group::new();
        let parent_id = 0;

        for n in 0..=5 {
            hexagon_side(&mut hex, parent_id, n);
        }

        let mut world = World::new();
        world.add_group(hex);
        
        world
    }

    pub fn three_sphere() -> World {
        let mut floor = Shape::default(Arc::new(Mutex::new(Plane::new())));
        let mut floor_material = Material::default();
        floor_material.set_color(Tuple::new_color(1.0, 0.9, 0.9));
        floor_material.set_specular(0.0);
        let floor_pattern = Pattern::stripe(Tuple::white(), Tuple::black(), PatternsKind::Checker);
        floor_material.set_pattern(floor_pattern);
        floor.set_material(floor_material.clone());
        floor.precompute_inverse_transformation();

        let mut middle = Shape::default(Arc::new(Mutex::new(Sphere::new())));
        middle.set_transformation(Transformation::translation(-0.5, 1.0, 0.5));
        let mut middle_material = Material::default();
        middle_material.set_color(Tuple::new_color(0.1, 1.0, 0.5));
        middle_material.set_diffuse(0.7);
        middle_material.set_specular(0.3);
        middle.set_material(middle_material);
        middle.precompute_inverse_transformation();

        let mut right = Shape::default(Arc::new(Mutex::new(Sphere::new())));
        right.set_transformation(
            Transformation::translation(1.5, 0.5, -0.5) * Transformation::scaling(0.5, 0.5, 0.5),
        );
        let mut right_material = Material::default();
        right_material.set_color(Tuple::new_color(0.5, 1.0, 0.1));
        right_material.set_diffuse(0.7);
        right_material.set_specular(0.3);
        right.set_material(right_material);
        right.precompute_inverse_transformation();

        let mut left = Shape::default(Arc::new(Mutex::new(Sphere::new())));
        left.set_transformation(
            Transformation::translation(-1.5, 0.33, -0.75) * Transformation::scaling(0.33, 0.33, 0.33),
        );
        let mut left_material = Material::default();
        left_material.set_color(Tuple::new_color(1.0, 0.8, 0.1));
        left_material.set_diffuse(0.7);
        left_material.set_specular(0.3);
        left.set_material(left_material);
        left.precompute_inverse_transformation();

        let mut world = World::new();

        let mut group = Group::new();
        group.add_node(left, Some(0));
        group.add_node(middle, Some(0));
        group.add_node(right, Some(0));

        world.add_shapes(&[floor]);
        world.add_group(group);
        
        return world
    }
}

fn hexagon_corner(parent_id: usize) -> Shape {
    let mut corner = Shape::default(Arc::new(Mutex::new(Sphere::new())));
    corner.set_transformation(Transformation::translation(0.0, 0.0, -1.0) * Transformation::scaling(0.25, 0.25, 0.25));
    corner.precompute_inverse_transformation();
    corner.set_parent_id(parent_id);

    return corner
}

fn hexagon_edge(parent_id: usize) -> Shape {
    let mut cylinder = Cylinder::new();
    cylinder.set_minimum(0.0);
    cylinder.set_maximum(1.0);
    let mut edge = Shape::default(Arc::new(Mutex::new(cylinder)));
    edge.set_transformation(Transformation::translation(0.0, 0.0, -1.0) * Transformation::rotation_y(-PI/6.0) * Transformation::rotation_z(-PI/2.0) * Transformation::scaling(0.25, 1.00, 0.25));
    edge.precompute_inverse_transformation();
    edge.set_parent_id(parent_id);

    return edge
}

fn hexagon_side(g: &mut Group, parent_id: usize, n: usize) {
    let rotation = Transformation::rotation_y(n as f64 * (PI/3.0));
    let matrix_id = g.add_matrix(rotation, Some(parent_id));
    g.add_node(hexagon_corner(matrix_id), Some(matrix_id));
    g.add_node(hexagon_edge(matrix_id), Some(matrix_id));
}