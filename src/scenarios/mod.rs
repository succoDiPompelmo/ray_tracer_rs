mod hexagon;
mod three_spheres;

use crate::world::World;

use self::{hexagon::Hexagon, three_spheres::ThreeSpheres};

pub struct Scenario {
    name: String,
    world: World,
}

impl Scenario {
    pub fn get(name: &str) -> Scenario {
        match name {
            "Hexagon" => Hexagon::new(),
            "Three Spheres" => ThreeSpheres::new(),
            _ => panic!("no scenario defined for name")
        }
    }

    pub fn list() -> Vec<String> {
        vec![Hexagon::name(), ThreeSpheres::name()]
    }

    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }

    pub fn get_world(&mut self) -> &mut World {
        &mut self.world
    }
}