mod camera;
mod canvas;
mod core;
mod margin;
mod materials;
mod rays;
mod scenarios;
mod shapes;

use std::f64::consts::PI;

use camera::Camera;
use core::transformations::Transformation;
use scenarios::Scenario;

use crate::{core::tuples::Tuple, scenarios::lights::PointLight};

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
