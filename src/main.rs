mod camera;
mod canvas;
mod intersections;
mod lights;
mod materials;
mod matrices;
mod patterns;
mod planes;
mod rays;
mod shapes;
mod spheres;
mod transformations;
mod tuples;
mod world;

use std::{
    f64::consts::PI,
    sync::{Arc, Mutex},
};

use camera::Camera;
use lights::PointLight;
use materials::Material;
use planes::Plane;
use shapes::Shape;
use spheres::Sphere;
use transformations::Transformation;
use world::World;

use crate::tuples::Tuple;

fn main() {
    let mut floor = Shape::default(Arc::new(Mutex::new(Plane::new())));
    let mut floor_material = Material::default();
    floor_material.set_color(Tuple::new_color(1.0, 0.9, 0.9));
    floor_material.set_specular(0.0);
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
    world.add_objects(&[floor, middle, right, left]);
    world.set_light(PointLight::new(
        Tuple::new_color(1.0, 1.0, 1.0),
        Tuple::new_point(-10.0, 10.0, -10.0),
    ));

    let mut camera = Camera::new(1000, 500, PI / 2.0);
    camera.set_transform(Transformation::view_transform(
        Tuple::new_point(0.0, 1.5, -5.0),
        Tuple::new_point(0.0, 1.0, 0.0),
        Tuple::new_vector(0.0, 1.0, 0.0),
    ));
    camera.precompute_inverse_transform();

    let canvas = camera.render(world);
    canvas.write_ppm_to_fs()
}
