use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

use crate::{
    intersections::{Computations, Intersection},
    lights::PointLight,
    materials::Material,
    rays::Ray,
    shapes::{Polygon, Shape},
    spheres::Sphere,
    transformations::Transformation,
    tuples::Tuple,
};

pub struct World {
    light: Option<PointLight>,
    objects: Vec<Shape>,
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

        let sphere = Sphere::new();
        let mut s1 = Shape::default(Arc::new(Mutex::new(sphere)));
        let mut m = Material::default();
        m.set_color(Tuple::new_color(0.8, 1.0, 0.6));
        m.set_diffuse(0.7);
        m.set_specular(0.2);
        s1.set_material(m);

        let sphere = Sphere::new();
        let mut s2 = Shape::default(Arc::new(Mutex::new(sphere)));
        s2.set_transformation(Transformation::scaling(0.5, 0.5, 0.5));

        World {
            light: Some(light),
            objects: vec![s1, s2],
        }
    }

    pub fn get_light(&self) -> Option<PointLight> {
        self.light.clone()
    }

    pub fn get_light_ref(&self) -> &PointLight {
        match &self.light {
            Some(light) => light,
            None => panic!("No light defined"),
        }
    }

    pub fn set_light(&mut self, light: PointLight) {
        self.light = Some(light);
    }

    pub fn get_objects(&self) -> Vec<Shape> {
        self.objects.to_vec()
    }

    pub fn add_objects(&mut self, shapes: &[Shape]) {
        for shape in shapes {
            self.objects.push(shape.clone());
        }
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

        let shadowed = self.is_shadowed(comps.get_over_point_ref());

        comps.get_object().get_material().lighting(
            light,
            comps.get_point_ref(),
            comps.get_eyev_ref(),
            comps.get_normalv_ref(),
            shadowed,
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

    fn is_shadowed(&self, point: &Tuple) -> bool {
        let v = self.get_light_ref().get_position_ref() - point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(point.clone(), direction);
        let intersections = self.intersect(&r);

        let h = Intersection::hit(&intersections);
        if let Some(hit) = h {
            if hit.get_t() < distance {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {

    use std::{borrow::BorrowMut, sync::Arc};

    use super::*;

    #[test]
    fn creating_a_world() {
        let w = World::new();

        assert!(w.get_light().is_none());
        assert!(w.get_objects().len() == 0);
    }

    #[test]
    fn the_default_world() {
        let l = PointLight::new(
            Tuple::new_color(1.0, 1.0, 1.0),
            Tuple::new_point(-10.0, 10.0, -10.0),
        );

        let sphere = Sphere::new();
        let mut s1 = Shape::default(Arc::new(Mutex::new(sphere)));
        let mut m = Material::default();
        m.set_color(Tuple::new_color(0.8, 1.0, 0.6));
        m.set_diffuse(0.7);
        m.set_specular(0.2);
        s1.set_material(m);

        let sphere = Sphere::new();
        let mut s2 = Shape::default(Arc::new(Mutex::new(sphere)));
        s2.set_transformation(Transformation::scaling(0.5, 0.5, 0.5));

        let w = World::default();

        assert!(w.get_light() == Some(l));
        assert!(w.get_objects().len() == 2);
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

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::default();
        let p = Tuple::new_point(0.0, 10.0, 0.0);

        assert!(!w.is_shadowed(&p));
    }

    #[test]
    fn shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = World::default();
        let p = Tuple::new_point(10.0, -10.0, 10.0);

        assert!(w.is_shadowed(&p));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = World::default();
        let p = Tuple::new_point(-20.0, 20.0, -20.0);

        assert!(!w.is_shadowed(&p));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = World::default();
        let p = Tuple::new_point(-2.0, 2.0, -2.0);

        assert!(!w.is_shadowed(&p));
    }

    #[test]
    fn intersection_in_shadow() {
        let mut w = World::default();
        w.set_light(PointLight::new(
            Tuple::new_color(1.0, 1.0, 1.0),
            Tuple::new_point(0.0, 0.0, -10.0),
        ));

        let sphere = Sphere::new();
        let mut s1 = Shape::default(Arc::new(Mutex::new(sphere)));

        let sphere = Sphere::new();
        let mut s2 = Shape::default(Arc::new(Mutex::new(sphere)));
        s2.set_transformation(Transformation::translation(0.0, 0.0, 10.0));

        w.add_objects(&[s1, s2.clone()]);

        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let i = Intersection::new(4.0, s2);
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);

        assert!(c == Tuple::new_color(0.1, 0.1, 0.1));
    }
}
