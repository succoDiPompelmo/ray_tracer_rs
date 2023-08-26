use crate::spheres::Sphere;

#[derive(Clone)]
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
        self.object
    }

    pub fn get_t(&self) -> f64 {
        self.t
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let t = 3.5;

        let intersection = Intersection::new(t, s);

        assert!(intersection.get_object() == s);
        assert!(intersection.get_t() == t);
    }

    #[test]
    fn aggregate_intersections() {
        let s = Sphere::new();

        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);

        let xs = Intersection::intersects(&[i1, i2]);

        assert!(xs.len() == 2);
        assert!(xs.get(0).unwrap().t == 1.0);
        assert!(xs.get(1).unwrap().t == 2.0);
    }
}
