use std::sync::{Arc, Mutex};

use crate::{
    core::transformations::Transformation,
    core::tuples::Tuple,
    materials::patterns::{Pattern, PatternsKind},
    materials::Material,
    shapes::groups::Group,
    shapes::planes::Plane,
    shapes::{cubes::Cube, Shape},
};

use super::{world::World, Scenario};

const NAME: &str = "Transparent Cube";
pub struct TransparentCube {}

impl TransparentCube {
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
    let floor_pattern = Pattern::stripe(Tuple::white(), Tuple::black(), PatternsKind::Ring);
    floor_material.set_pattern(floor_pattern);
    floor.set_material(floor_material.clone());
    floor.precompute_inverse_transformation();

    let mut cube = Shape::default(Arc::new(Mutex::new(Cube::new())));
    cube.set_transformation(Transformation::translation(-0.5, 1.0, 0.5));
    let mut cube_material = Material::default();
    cube_material.set_color(Tuple::new_color(0.1, 1.0, 0.5));
    cube_material.set_diffuse(0.7);
    cube_material.set_specular(0.3);
    cube_material.set_transparency(0.6);
    cube_material.set_refractive_index(0.8);
    cube.set_material(cube_material);
    cube.precompute_inverse_transformation();

    let mut world = World::new();

    let mut group = Group::new();
    group.add_node(cube, Some(0));

    world.add_shapes(&[floor]);
    world.add_group(group);

    world
}
