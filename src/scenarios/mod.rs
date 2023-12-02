mod hexagon;
pub mod lights;
mod three_spheres;
pub mod world;
mod transparent_cube;

use self::{hexagon::Hexagon, three_spheres::ThreeSpheres, world::World, transparent_cube::TransparentCube};

pub struct Scenario {
    world: World,
}

impl Scenario {
    pub fn get(name: &str) -> Scenario {
        match name {
            "Hexagon" => Hexagon::new(),
            "Three Spheres" => ThreeSpheres::new(),
            "Transparent Cube" => TransparentCube::new(),
            _ => panic!("no scenario defined for name"),
        }
    }

    pub fn list() -> Vec<String> {
        vec![Hexagon::name(), ThreeSpheres::name(), TransparentCube::name()]
    }

    pub fn get_world(&mut self) -> &mut World {
        &mut self.world
    }
}
