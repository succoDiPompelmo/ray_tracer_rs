mod hexagon;
pub mod lights;
mod three_spheres;
mod transparent_cube;
pub mod world;

use self::{
    hexagon::Hexagon, three_spheres::ThreeSpheres, transparent_cube::TransparentCube, world::World,
};

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
        vec![
            Hexagon::name(),
            ThreeSpheres::name(),
            TransparentCube::name(),
        ]
    }

    pub fn get_world(&mut self) -> &mut World {
        &mut self.world
    }
}
