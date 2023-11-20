mod camera;
mod canvas;
mod core;
mod intersections;
mod lights;
mod margin;
mod materials;
mod rays;
mod scenarios;
mod shapes;
mod world;

use std::f64::consts::PI;

use camera::Camera;
use core::transformations::Transformation;
use lights::PointLight;
use scenarios::Scenario;

use crate::core::tuples::Tuple;

fn main() {
    println!("{:?}", Scenario::list());

    let mut scenario = Scenario::get("Hexagon");

    scenario.get_world().set_light(PointLight::new(
        Tuple::white(),
        Tuple::new_point(-5.0, 10.0, -10.0),
    ));

    let mut camera = Camera::new(1000, 500, PI / 2.0);
    camera.set_transform(Transformation::view_transform(
        Tuple::new_point(0.0, 1.5, -5.0),
        Tuple::new_point(0.0, 1.0, 0.0),
        Tuple::new_vector(0.0, 1.0, 0.0),
    ));
    camera.precompute_inverse_transform();

    let canvas = camera.render(scenario.get_world());
    canvas.save(scenario.get_name());
}
