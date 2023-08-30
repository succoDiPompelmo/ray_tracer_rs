use crate::tuples::Tuple;

pub struct PointLight {
    intensity: Tuple,
    position: Tuple,
}

impl PointLight {
    pub fn new(intensity: Tuple, position: Tuple) -> PointLight {
        PointLight {
            intensity,
            position,
        }
    }

    pub fn get_intensity(&self) -> Tuple {
        self.intensity
    }

    pub fn get_position(&self) -> Tuple {
        self.position
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = Tuple::new_color(1.0, 1.0, 1.0);
        let position = Tuple::new_point(0.0, 0.0, 0.0);

        let light = PointLight::new(intensity, position);

        assert!(light.get_position() == position);
        assert!(light.get_intensity() == intensity);
    }
}
