use std::f64::consts::PI;

use crate::{canvas::Canvas, transformations::Transformation, tuples::Tuple};

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

    pub fn get_x(&self) -> f64 {
        self.position.x
    }

    pub fn get_y(&self) -> f64 {
        self.position.y
    }

    pub fn to_canvas_coordinate(&self, canvas: &Canvas) -> Tuple {
        Tuple::new_point(
            self.position.x,
            canvas.height() as f64 - self.position.y,
            self.position.z,
        )
    }
}

pub struct Clock {
    position: Tuple,
    ticks: usize,
}

impl Clock {
    pub fn new(r: f64, ticks: usize) -> Clock {
        Clock {
            position: Tuple::new_point(r, 0.0, 0.0),
            ticks,
        }
    }

    pub fn tick(&self) -> Clock {
        let r = Transformation::rotation_z((2.0 * PI) / self.ticks as f64);

        Clock {
            position: &r * &self.position,
            ticks: self.ticks,
        }
    }

    pub fn get_x(&self) -> f64 {
        self.position.x
    }

    pub fn get_y(&self) -> f64 {
        self.position.y
    }
}
