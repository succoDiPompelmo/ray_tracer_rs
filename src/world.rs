use crate::{
    intersections::{Computations, Intersection},
    lights::PointLight,
    materials::Material,
    rays::Ray,
    spheres::Sphere,
    transformations::Transformation,
    tuples::Tuple,
};

pub struct World {
    light: Option<PointLight>,
    objects: Vec<Sphere>,
}

impl World {
    pub fn new() -> World {
        World {
            light: None,
            objects: vec![],
        }
    }

    pub fn default() -> World {
        let light = PointLight::new(
            Tuple::new_color(1.0, 1.0, 1.0),
            Tuple::new_point(-10.0, 10.0, -10.0),
        );

        let mut s1 = Sphere::new();
        let mut m = Material::default();
        m.set_color(Tuple::new_color(0.8, 1.0, 0.6));
        m.set_diffuse(0.7);
        m.set_specular(0.2);
        s1.set_material(m);

        let mut s2 = Sphere::new();
        s2.set_transformation(Transformation::scaling(0.5, 0.5, 0.5));

        World {
            light: Some(light),
            objects: vec![s1, s2],
        }
    }

    pub fn get_light(&self) -> Option<PointLight> {
        self.light.clone()
    }

    pub fn set_light(&mut self, light: PointLight) {
        self.light = Some(light);
    }

    pub fn get_objects(&self) -> Vec<Sphere> {
        self.objects.to_vec()
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections = vec![];

        for object in &self.objects {
            let xs = object.intersect(&ray);
            intersections.extend(xs);
        }

        intersections.sort_by(|a, b| a.get_t().partial_cmp(&b.get_t()).unwrap());
        intersections
    }

    pub fn shade_hit(&self, comps: &Computations) -> Tuple {
        let light = self.light.as_ref().unwrap();
        comps.get_object().get_material().lighting(
            light,
            comps.get_point(),
            comps.get_eyev(),
            comps.get_normalv(),
        )
    }

    pub fn color_at(&self, ray: &Ray) -> Tuple {
        let intersections = self.intersect(ray);

        match Intersection::hit(&intersections) {
            None => Tuple::new_color(0.0, 0.0, 0.0),
            Some(hit) => {
                let comps = hit.prepare_computations(ray);
                self.shade_hit(&comps)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use std::borrow::BorrowMut;

    use super::*;

    #[test]
    fn creating_a_world() {
        let w = World::new();

        assert!(w.get_light().is_none());
        assert!(w.get_objects() == vec![]);
    }

    #[test]
    fn the_default_world() {
        let l = PointLight::new(
            Tuple::new_color(1.0, 1.0, 1.0),
            Tuple::new_point(-10.0, 10.0, -10.0),
        );

        let mut s1 = Sphere::new();
        let mut m = Material::default();
        m.set_color(Tuple::new_color(0.8, 1.0, 0.6));
        m.set_diffuse(0.7);
        m.set_specular(0.2);
        s1.set_material(m);

        let mut s2 = Sphere::new();
        s2.set_transformation(Transformation::scaling(0.5, 0.5, 0.5));

        let w = World::default();

        assert!(w.get_light() == Some(l));
        assert!(w.get_objects().contains(&s1));
        assert!(w.get_objects().contains(&s2));
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = World::default();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );

        let xs = w.intersect(&r);

        assert!(xs.len() == 4);
        assert!(xs.get(0).unwrap().get_t() == 4.0);
        assert!(xs.get(1).unwrap().get_t() == 4.5);
        assert!(xs.get(2).unwrap().get_t() == 5.5);
        assert!(xs.get(3).unwrap().get_t() == 6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let w = World::default();

        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let objects = w.get_objects();

        let i = Intersection::new(4.0, objects.get(0).unwrap().clone());
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);
        assert!(
            c == Tuple::new_color(
                0.38066119308103435,
                0.47582649135129296,
                0.28549589481077575
            )
        );
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = World::default();
        w.set_light(PointLight::new(
            Tuple::new_color(1.0, 1.0, 1.0),
            Tuple::new_point(0.0, 0.25, 0.0),
        ));

        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let objects = w.get_objects();

        let i = Intersection::new(0.5, objects.get(1).unwrap().clone());
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);

        assert!(c == Tuple::new_color(0.9049844720832575, 0.9049844720832575, 0.9049844720832575));
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = World::default();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );
        let c = w.color_at(&r);

        assert!(c == Tuple::new_color(0.0, 0.0, 0.0));
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = World::default();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let c = w.color_at(&r);

        assert!(
            c == Tuple::new_color(
                0.38066119308103435,
                0.47582649135129296,
                0.28549589481077575
            )
        );
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let mut w = World::default();

        w.objects.get_mut(0).unwrap().material.set_ambient(1.0);
        w.objects.get_mut(1).unwrap().material.set_ambient(1.0);

        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.75),
            Tuple::new_vector(0.0, 0.0, -1.0),
        );

        assert!(w.objects.get(1).unwrap().material.get_color() == w.color_at(&r));
    }
}
