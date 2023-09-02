use crate::{lights::PointLight, tuples::Tuple};

#[derive(Clone, Debug, PartialEq)]
pub struct Material {
    color: Tuple,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
}

impl Material {
    pub fn default() -> Material {
        Material {
            color: Tuple::new_color(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn get_color(&self) -> Tuple {
        self.color
    }

    fn get_ambient(&self) -> f64 {
        self.ambient
    }

    fn get_diffuse(&self) -> f64 {
        self.diffuse
    }

    pub fn set_diffuse(&mut self, diffuse: f64) {
        self.diffuse = diffuse
    }

    fn get_specular(&self) -> f64 {
        self.specular
    }

    pub fn set_specular(&mut self, specular: f64) {
        self.specular = specular
    }

    fn get_shininess(&self) -> f64 {
        self.shininess
    }

    pub fn set_color(&mut self, color: Tuple) {
        self.color = color
    }

    pub fn set_ambient(&mut self, ambient: f64) {
        self.ambient = ambient;
    }

    pub fn lighting(&self, light: &PointLight, point: Tuple, eyev: Tuple, normalv: Tuple) -> Tuple {
        let effective_color = self.color.hadamard_product(&light.get_intensity());
        let lightv = (light.get_position() - point).normalize();

        let ambient = effective_color * self.ambient;

        let light_dot_normal = lightv.dot(&normalv);
        let mut diffuse = Tuple::new_color(0.0, 0.0, 0.0);
        let mut specular = Tuple::new_color(0.0, 0.0, 0.0);

        if light_dot_normal > 0.0 {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = reflectv.dot(&eyev);

            if reflect_dot_eye > 0.0 {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.get_intensity() * self.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {

    use crate::lights::PointLight;

    use super::*;

    #[test]
    fn default_material() {
        let m = Material::default();

        assert!(m.get_color() == Tuple::new_color(1.0, 1.0, 1.0));
        assert!(m.get_ambient() == 0.1);
        assert!(m.get_diffuse() == 0.9);
        assert!(m.get_specular() == 0.9);
        assert!(m.get_shininess() == 200.0);
    }

    #[test]
    fn lighting_with_eye_between_the_light_and_the_surface() {
        let m = Material::default();
        let position = Tuple::new_point(0.0, 0.0, 0.0);

        let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(
            Tuple::new_color(1.0, 1.0, 1.0),
            Tuple::new_point(0.0, 0.0, -10.0),
        );

        let r = m.lighting(&light, position, eyev, normalv);
        assert!(r == Tuple::new_color(1.9, 1.9, 1.9))
    }

    #[test]
    fn lighting_with_eye_between_the_light_and_the_surface_with_45_degree_eye_offset() {
        let m = Material::default();
        let position = Tuple::new_point(0.0, 0.0, 0.0);

        let eyev = Tuple::new_vector(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(
            Tuple::new_color(1.0, 1.0, 1.0),
            Tuple::new_point(0.0, 0.0, -10.0),
        );

        let r = m.lighting(&light, position, eyev, normalv);
        assert!(r == Tuple::new_color(1.0, 1.0, 1.0))
    }

    #[test]
    fn lighting_with_eye_opposite_the_surface_and_45_degree_light_offset() {
        let m = Material::default();
        let position = Tuple::new_point(0.0, 0.0, 0.0);

        let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(
            Tuple::new_color(1.0, 1.0, 1.0),
            Tuple::new_point(0.0, 10.0, -10.0),
        );

        let r = m.lighting(&light, position, eyev, normalv);
        let value = 0.1 + 0.9 * 2.0_f64.sqrt() / 2.0 + 0.0;
        assert!(r == Tuple::new_color(value, value, value))
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::default();
        let position = Tuple::new_point(0.0, 0.0, 0.0);

        let eyev = Tuple::new_vector(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(
            Tuple::new_color(1.0, 1.0, 1.0),
            Tuple::new_point(0.0, 10.0, -10.0),
        );

        let r = m.lighting(&light, position, eyev, normalv);
        let value = 0.1 + 0.9 * 2.0_f64.sqrt() / 2.0 + 0.9;
        assert!(r == Tuple::new_color(value, value, value))
    }

    #[test]
    fn lighting_with_light_behind_the_surface() {
        let m = Material::default();
        let position = Tuple::new_point(0.0, 0.0, 0.0);

        let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(
            Tuple::new_color(1.0, 1.0, 1.0),
            Tuple::new_point(0.0, 0.0, 10.0),
        );

        let r = m.lighting(&light, position, eyev, normalv);
        assert!(r == Tuple::new_color(0.1, 0.1, 0.1))
    }
}
