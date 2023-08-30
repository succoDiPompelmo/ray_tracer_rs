use crate::tuples::Tuple;

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

    fn get_color(&self) -> Tuple {
        self.color
    }

    fn get_ambient(&self) -> f64 {
        self.ambient
    }

    fn get_diffuse(&self) -> f64 {
        self.diffuse
    }

    fn get_specular(&self) -> f64 {
        self.specular
    }

    fn get_shininess(&self) -> f64 {
        self.shininess
    }

    pub fn set_ambient(&mut self, ambient: f64) {
        self.ambient = ambient;
    }
}

#[cfg(test)]
mod tests {

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
}
