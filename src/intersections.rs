use crate::spheres::Sphere;

#[derive(Clone, Debug, PartialEq)]
pub struct Intersection {
    t: f64,
    object: Sphere,
}

impl Intersection {
    pub fn new(t: f64, object: Sphere) -> Intersection {
        Intersection { t, object }
    }

    pub fn intersects(intersections: &[Intersection]) -> Vec<Intersection> {
        intersections.to_vec()
    }

    pub fn get_object(&self) -> Sphere {
        self.object.clone()
    }

    pub fn get_t(&self) -> f64 {
        self.t
    }

    pub fn hit(intersections: &[Intersection]) -> Option<Intersection> {
        let mut hit = None;

        for intersection in intersections {
            if intersection.get_t() > 0.0 {
                if hit.is_none() {
                    hit = Some(intersection);
                }

                if let Some(hit_intersection) = hit {
                    if hit_intersection.get_t() > intersection.get_t() {
                        hit = Some(intersection)
                    }
                }
            }
        }

        hit.cloned()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let t = 3.5;

        let intersection = Intersection::new(t, s.clone());

        assert!(intersection.get_object() == s);
        assert!(intersection.get_t() == t);
    }

    #[test]
    fn aggregate_intersections() {
        let s = Sphere::new();

        let i1 = Intersection::new(1.0, s.clone());
        let i2 = Intersection::new(2.0, s);

        let xs = Intersection::intersects(&[i1, i2]);

        assert!(xs.len() == 2);
        assert!(xs.get(0).unwrap().t == 1.0);
        assert!(xs.get(1).unwrap().t == 2.0);
    }

    #[test]
    fn hit_when_all_intersections_are_positives() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, s.clone());
        let i2 = Intersection::new(2.0, s);

        let xs = Intersection::intersects(&[i1.clone(), i2]);

        assert!(Intersection::hit(&xs) == Some(i1));
    }

    #[test]
    fn hit_when_some_intersections_are_negatives() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, s.clone());
        let i2 = Intersection::new(1.0, s);

        let xs = Intersection::intersects(&[i1, i2.clone()]);

        assert!(Intersection::hit(&xs) == Some(i2));
    }

    #[test]
    fn hit_when_all_intersections_are_negatives() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, s.clone());
        let i2 = Intersection::new(-1.0, s);

        let xs = Intersection::intersects(&[i1, i2]);

        assert!(Intersection::hit(&xs) == None);
    }

    #[test]
    fn hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, s.clone());
        let i2 = Intersection::new(7.0, s.clone());
        let i3 = Intersection::new(-3.0, s.clone());
        let i4 = Intersection::new(2.0, s.clone());

        let xs = Intersection::intersects(&[i1, i2, i3, i4.clone()]);

        assert!(Intersection::hit(&xs) == Some(i4));
    }
}
