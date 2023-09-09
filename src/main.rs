mod camera;
mod canvas;
mod intersections;
mod lights;
mod materials;
mod matrices;
mod play;
mod rays;
mod shapes;
mod spheres;
mod transformations;
mod tuples;
mod world;

use std::f64::consts::PI;

use camera::Camera;
use lights::PointLight;
use materials::Material;
use spheres::Sphere;
use transformations::Transformation;
use world::World;

use crate::tuples::Tuple;

fn main() {
    let mut floor = Sphere::new();
    floor.set_transformation(Transformation::scaling(10.0, 0.01, 10.0));
    let mut floor_material = Material::default();
    floor_material.set_color(Tuple::new_color(1.0, 0.9, 0.9));
    floor_material.set_specular(0.0);
    floor.set_material(floor_material.clone());

    let mut left_wall = Sphere::new();
    left_wall.set_transformation(
        Transformation::translation(0.0, 0.0, 5.0)
            * Transformation::rotation_y(-PI / 4.0)
            * Transformation::rotation_x(PI / 2.0)
            * Transformation::scaling(10.0, 0.01, 10.0),
    );
    left_wall.set_material(floor_material.clone());

    let mut right_wall = Sphere::new();
    right_wall.set_transformation(
        Transformation::translation(0.0, 0.0, 5.0)
            * Transformation::rotation_y(PI / 4.0)
            * Transformation::rotation_x(PI / 2.0)
            * Transformation::scaling(10.0, 0.01, 10.0),
    );
    right_wall.set_material(floor_material.clone());

    let mut middle = Sphere::new();
    middle.set_transformation(Transformation::translation(-0.5, 1.0, 0.5));
    let mut middle_material = Material::default();
    middle_material.set_color(Tuple::new_color(0.1, 1.0, 0.5));
    middle_material.set_diffuse(0.7);
    middle_material.set_specular(0.3);
    middle.set_material(middle_material);

    let mut right = Sphere::new();
    right.set_transformation(
        Transformation::translation(1.5, 0.5, -0.5) * Transformation::scaling(0.5, 0.5, 0.5),
    );
    let mut right_material = Material::default();
    right_material.set_color(Tuple::new_color(0.5, 1.0, 0.1));
    right_material.set_diffuse(0.7);
    right_material.set_specular(0.3);
    right.set_material(right_material);

    let mut left = Sphere::new();
    left.set_transformation(
        Transformation::translation(-1.5, 0.33, -0.75) * Transformation::scaling(0.33, 0.33, 0.33),
    );
    let mut left_material = Material::default();
    left_material.set_color(Tuple::new_color(1.0, 0.8, 0.1));
    left_material.set_diffuse(0.7);
    left_material.set_specular(0.3);
    left.set_material(left_material);

    let mut world = World::new();
    world.add_objects(&[floor, right_wall, left_wall, middle, right, left]);
    world.set_light(PointLight::new(
        Tuple::new_color(1.0, 1.0, 1.0),
        Tuple::new_point(-10.0, 10.0, -10.0),
    ));

    let mut camera = Camera::new(800, 400, PI / 2.0);
    camera.set_transform(Transformation::view_transform(
        Tuple::new_point(0.0, 1.5, -5.0),
        Tuple::new_point(0.0, 1.0, 0.0),
        Tuple::new_vector(0.0, 1.0, 0.0),
    ));

    let canvas = camera.render(world);
    canvas.write_ppm_to_fs()
}
