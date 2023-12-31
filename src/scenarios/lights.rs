use crate::core::tuples::Tuple;

#[derive(Clone, Debug, PartialEq)]
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
        self.intensity.clone()
    }

    pub fn get_position_ref(&self) -> &Tuple {
        &self.position
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = Tuple::white();
        let position = Tuple::new_point(0.0, 0.0, 0.0);

        let light = PointLight::new(intensity.clone(), position.clone());

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
