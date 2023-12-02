use std::sync::{Arc, Mutex};

use crate::{
    core::transformations::Transformation,
    core::tuples::Tuple,
    materials::patterns::{Pattern, PatternsKind},
    materials::Material,
    shapes::groups::Group,
    shapes::planes::Plane,
    shapes::spheres::Sphere,
    shapes::Shape,
};

use super::{world::World, Scenario};

const NAME: &str = "Three Spheres";
pub struct ThreeSpheres {}

impl ThreeSpheres {
    pub fn new() -> Scenario {
        Scenario { world: draw() }
    }

    pub fn name() -> String {
        NAME.to_owned()
    }
}

pub fn draw() -> World {
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

    world
}
