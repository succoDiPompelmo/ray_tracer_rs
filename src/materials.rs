use crate::{lights::PointLight, patterns::Pattern, shapes::Shape, tuples::Tuple};

#[derive(Clone, Debug)]
pub struct Material {
    color: Tuple,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
    reflective: f64,
    transparency: f64,
    refractive_index: f64,
    pattern: Option<Pattern>,
}

impl Material {
    pub fn default() -> Material {
        Material {
            color: Tuple::white(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
            pattern: None,
        }
    }

    #[cfg(test)]
    pub fn get_color(&self) -> Tuple {
        self.color
    }

    pub fn get_reflective(&self) -> f64 {
        self.reflective
    }

    #[cfg(test)]
    pub fn get_transparency(&self) -> f64 {
        self.transparency
    }

    pub fn get_refractive_index(&self) -> f64 {
        self.refractive_index
    }

    pub fn set_diffuse(&mut self, diffuse: f64) {
        self.diffuse = diffuse
    }

    pub fn set_specular(&mut self, specular: f64) {
        self.specular = specular
    }

    pub fn set_color(&mut self, color: Tuple) {
        self.color = color
    }

    pub fn set_pattern(&mut self, pattern: Pattern) {
        self.pattern = Some(pattern)
    }

    #[cfg(test)]
    pub fn set_ambient(&mut self, ambient: f64) {
        self.ambient = ambient;
    }

    #[cfg(test)]
    pub fn set_reflective(&mut self, reflective: f64) {
        self.reflective = reflective
    }

    #[cfg(test)]
    pub fn set_transparency(&mut self, transparency: f64) {
        self.transparency = transparency
    }

    #[cfg(test)]
    pub fn set_refractive_index(&mut self, refractive_index: f64) {
        self.refractive_index = refractive_index
    }

    pub fn lighting(
        &self,
        object: &Shape,
        light: &PointLight,
        point: &Tuple,
        eyev: &Tuple,
        normalv: &Tuple,
        in_shadow: bool,
    ) -> Tuple {
        let color = match &self.pattern {
            Some(p) => p.stripe_at_object(object, point),
            None => self.color,
        };

        let effective_color = color.hadamard_product(&light.get_intensity());
        let lightv = (light.get_position_ref() - point).normalize();

        let ambient = effective_color * self.ambient;

        if in_shadow {
            return ambient;
        }

        let light_dot_normal = lightv.dot(normalv);
        let mut diffuse = Tuple::black();
        let mut specular = Tuple::black();

        if light_dot_normal > 0.0 {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);

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

    use std::sync::{Arc, Mutex};

    use float_cmp::{ApproxEq, F64Margin};

    use crate::{lights::PointLight, patterns::PatternsKind, spheres::Sphere};

    use super::*;

    #[test]
    fn default_material() {
        let m = Material::default();

        assert_eq!(m.get_color(), Tuple::white());
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting_with_eye_between_the_light_and_the_surface() {
        let m = Material::default();
        let point = Tuple::new_point(0.0, 0.0, 0.0);

        let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::white(), Tuple::new_point(0.0, 0.0, -10.0));
        let in_shadow = false;
        let object = Shape::default(Arc::new(Mutex::new(Sphere::new())));

        let r = m.lighting(&object, &light, &point, &eyev, &normalv, in_shadow);
        assert_eq!(r, Tuple::new_color(1.9, 1.9, 1.9))
    }

    #[test]
    fn lighting_with_eye_between_the_light_and_the_surface_with_45_degree_eye_offset() {
        let m = Material::default();
        let point = Tuple::new_point(0.0, 0.0, 0.0);

        let eyev = Tuple::new_vector(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::white(), Tuple::new_point(0.0, 0.0, -10.0));
        let in_shadow = false;
        let object = Shape::default(Arc::new(Mutex::new(Sphere::new())));

        let r = m.lighting(&object, &light, &point, &eyev, &normalv, in_shadow);
        assert_eq!(r, Tuple::white())
    }

    #[test]
    fn lighting_with_eye_opposite_the_surface_and_45_degree_light_offset() {
        let m = Material::default();
        let point = Tuple::new_point(0.0, 0.0, 0.0);

        let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::white(), Tuple::new_point(0.0, 10.0, -10.0));
        let in_shadow = false;
        let object = Shape::default(Arc::new(Mutex::new(Sphere::new())));

        let r = m.lighting(&object, &light, &point, &eyev, &normalv, in_shadow);
        let value = 0.1 + 0.9 * 2.0_f64.sqrt() / 2.0 + 0.0;
        assert_eq!(r, Tuple::new_color(value, value, value))
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::default();
        let point = Tuple::new_point(0.0, 0.0, 0.0);

        let eyev = Tuple::new_vector(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::white(), Tuple::new_point(0.0, 10.0, -10.0));
        let in_shadow = false;
        let object = Shape::default(Arc::new(Mutex::new(Sphere::new())));

        let r = m.lighting(&object, &light, &point, &eyev, &normalv, in_shadow);
        let value = 0.1 + 0.9 * 2.0_f64.sqrt() / 2.0 + 0.9;
        assert_eq!(r, Tuple::new_color(value, value, value))
    }

    #[test]
    fn lighting_with_light_behind_the_surface() {
        let m = Material::default();
        let point = Tuple::new_point(0.0, 0.0, 0.0);

        let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::white(), Tuple::new_point(0.0, 0.0, 10.0));
        let in_shadow = false;
        let object = Shape::default(Arc::new(Mutex::new(Sphere::new())));

        let r = m.lighting(&object, &light, &point, &eyev, &normalv, in_shadow);
        assert_eq!(r, Tuple::new_color(0.1, 0.1, 0.1))
    }

    #[test]
    fn lighting_with_surface_in_shadow() {
        let m = Material::default();
        let point = Tuple::new_point(0.0, 0.0, 0.0);

        let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::white(), Tuple::new_point(0.0, 0.0, -10.0));
        let in_shadow = true;
        let object = Shape::default(Arc::new(Mutex::new(Sphere::new())));

        let result = m.lighting(&object, &light, &point, &eyev, &normalv, in_shadow);
        assert_eq!(result, Tuple::new_color(0.1, 0.1, 0.1))
    }

    #[test]
    fn lighting_with_a_pattern_applied() {
        let mut m = Material::default();
        m.pattern = Some(Pattern::stripe(
            Tuple::white(),
            Tuple::black(),
            PatternsKind::Stripe,
        ));
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;

        let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::white(), Tuple::new_point(0.0, 0.0, -10.0));
        let object = Shape::default(Arc::new(Mutex::new(Sphere::new())));

        let c1 = m.lighting(
            &object,
            &light,
            &Tuple::new_point(0.9, 0.0, 0.0),
            &eyev,
            &normalv,
            false,
        );
        let c2 = m.lighting(
            &object,
            &light,
            &Tuple::new_point(1.1, 0.0, 0.0),
            &eyev,
            &normalv,
            false,
        );

        assert_eq!(Tuple::white(), c1);
        assert_eq!(Tuple::black(), c2);
    }

    #[test]
    fn reflectivity_for_the_default_material() {
        let material = Material::default();

        let margin = F64Margin {
            ulps: 2,
            epsilon: 1e-14,
        };

        assert!(material.reflective.approx_eq(0.0, margin));
    }

    #[test]
    fn transparency_and_refractive_index_for_default_material() {
        let material = Material::default();

        let margin = F64Margin {
            ulps: 2,
            epsilon: 1e-14,
        };

        assert!(material.transparency.approx_eq(0.0, margin));
        assert!(material.refractive_index.approx_eq(1.0, margin));
    }
}
