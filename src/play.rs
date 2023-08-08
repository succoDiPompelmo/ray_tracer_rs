use crate::tuples::Tuple;

pub struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

#[derive(Debug)]
pub struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

impl Environment {
    pub fn new(gravity: Tuple, wind: Tuple) -> Environment {
        Environment { gravity, wind }
    }

    pub fn tick(&self, proj: Projectile) -> Projectile {
        let position = proj.position + proj.velocity;
        let velocity = proj.velocity + self.gravity + self.wind;

        Projectile { position, velocity }
    }
}

impl Projectile {
    pub fn new(position: Tuple, velocity: Tuple) -> Projectile {
        Projectile { position, velocity }
    }

    pub fn get_y(&self) -> f32 {
        self.position.y
    }
}
