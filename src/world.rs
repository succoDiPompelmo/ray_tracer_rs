use float_cmp::{ApproxEq, F64Margin};

use crate::{
    intersections::{Computations, Intersection},
    lights::PointLight,
    rays::Ray,
    shapes::Shape,
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

    pub fn get_light_ref(&self) -> &PointLight {
        match &self.light {
            Some(light) => light,
            None => panic!("No light defined"),
        }
    }

    pub fn set_light(&mut self, light: PointLight) {
        self.light = Some(light);
    }

    pub fn add_objects(&mut self, shapes: &[Shape]) {
        for shape in shapes {
            self.objects.push(shape.clone());
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections = vec![];

        for object in &self.objects {
            let xs = object.intersect(ray);
            intersections.extend(xs);
        }

        intersections.sort_by(|a, b| a.get_t().partial_cmp(&b.get_t()).unwrap());
        intersections
    }

    pub fn shade_hit(&self, comps: &Computations) -> Tuple {
        let light = self.light.as_ref().unwrap();

        let shadowed = self.is_shadowed(comps.get_over_point_ref());

        let surface = comps.get_object().get_material().lighting(
            &comps.get_object(),
            light,
            comps.get_point_ref(),
            comps.get_eyev_ref(),
            comps.get_normalv_ref(),
            shadowed,
        );

        let reflected = self.reflected_color(comps);

        surface + reflected
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

        let r = Ray::new(*point, direction);
        let intersections = self.intersect(&r);

        let h = Intersection::hit(&intersections);
        if let Some(hit) = h {
            if hit.get_t() < distance {
                return true;
            }
        }

        false
    }

    pub fn reflected_color(&self, comps: &Computations) -> Tuple {
        let margin = F64Margin {
            ulps: 2,
            epsilon: 1e-14,
        };

        if comps
            .get_object()
            .get_material()
            .get_reflective()
            .approx_eq(0.0, margin)
        {
            return Tuple::new_color(0.0, 0.0, 0.0);
        }

        let reflected_ray = Ray::new(*comps.get_over_point_ref(), *comps.get_reflectv());
        let color = self.color_at(&reflected_ray);

        return color * comps.get_object().get_material().get_reflective();
    }
}

#[cfg(test)]
mod tests {

    use std::sync::{Arc, Mutex};

    use crate::{
        materials::Material, planes::Plane, spheres::Sphere, transformations::Transformation,
    };

    use super::*;

    impl World {
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
    }

    #[test]
    fn creating_a_world() {
        let w = World::new();

        assert!(w.light.is_none());
        assert!(w.objects.len() == 0);
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

        assert!(w.light == Some(l));
        assert!(w.objects.len() == 2);
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
        let objects = w.objects.to_vec();

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
        let objects = w.objects.to_vec();

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
        let s1 = Shape::default(Arc::new(Mutex::new(sphere)));

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

    #[test]
    fn the_reflected_color_for_a_nonreflective_material() {
        let mut w = World::default();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );

        let shape = w.objects.get_mut(1).unwrap();
        shape.material.set_ambient(1.0);

        let i = Intersection::new(1.0, shape.clone());
        let comps = i.prepare_computations(&r);
        let color = w.reflected_color(&comps);

        assert_eq!(color, Tuple::new_color(0.0, 0.0, 0.0));
    }

    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let mut w = World::default();

        let plane = Plane::new();
        let mut s = Shape::default(Arc::new(Mutex::new(plane)));

        let mut plane_material = Material::default();
        plane_material.set_reflective(0.5);

        s.set_material(plane_material);
        s.set_transformation(Transformation::translation(0.0, -1.0, 0.0));
        w.add_objects(&[s.clone()]);

        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -3.0),
            Tuple::new_vector(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );

        let i = Intersection::new(2.0_f64.sqrt(), s.clone());
        let comps = i.prepare_computations(&r);
        let color = w.reflected_color(&comps);

        assert_eq!(
            color,
            Tuple::new_color(0.1903307689243628, 0.23791346115545348, 0.1427480766932721)
        );
    }

    #[test]
    fn shade_hit_with_a_reflective_material() {
        let mut w = World::default();

        let plane = Plane::new();
        let mut s = Shape::default(Arc::new(Mutex::new(plane)));

        let mut plane_material = Material::default();
        plane_material.set_reflective(0.5);

        s.set_material(plane_material);
        s.set_transformation(Transformation::translation(0.0, -1.0, 0.0));
        w.add_objects(&[s.clone()]);

        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -3.0),
            Tuple::new_vector(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );

        let i = Intersection::new(2.0_f64.sqrt(), s.clone());
        let comps = i.prepare_computations(&r);
        let color = w.shade_hit(&comps);

        assert_eq!(
            color,
            Tuple::new_color(0.8767561579058643, 0.9243388501369549, 0.8291734656747736)
        );
    }
}
