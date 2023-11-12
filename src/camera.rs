use crate::{canvas::Canvas, matrices::Matrix, rays::Ray, tuples::Tuple, world::World};

pub struct Camera {
    hsize: usize,
    vsize: usize,
    _field_of_view: f64,
    transform: Matrix,
    inverse_transform: Option<Matrix>,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Camera {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;

        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = (half_width * 2.0) / hsize as f64;

        Camera {
            hsize,
            vsize,
            _field_of_view: field_of_view,
            transform: Matrix::identity(4),
            inverse_transform: None,
            half_height,
            half_width,
            pixel_size,
        }
    }

    fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let inverse_transform = match &self.inverse_transform {
            Some(matrix) => matrix.clone(),
            None => self.transform.invert(),
        };

        // Remember that canvas is at z = -1
        let pixel = &inverse_transform * &Tuple::new_point(world_x, world_y, -1.0);
        let origin = &inverse_transform * &Tuple::new_point(0.0, 0.0, 0.0);
        let direction = (&pixel - &origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &mut World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);
        let mut pixels = vec![];

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                pixels.push((x, y));
            }
        }

        for (x, y) in pixels {
            let ray = self.ray_for_pixel(x, y);
            let color = world.color_at(&ray, 5);

            image.write_pixel(color, x as isize, y as isize);
        }

        image
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    pub fn precompute_inverse_transform(&mut self) {
        self.inverse_transform = Some(self.transform.invert());
    }
}

#[cfg(test)]
mod tests {

    use std::f64::consts::PI;

    use crate::{
        canvas::Canvas, margin::Margin, transformations::Transformation, tuples::Tuple,
        world::World,
    };
    use float_cmp::ApproxEq;

    use super::*;

    #[test]
    fn construct_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;

        let c = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(c.hsize, hsize);
        assert_eq!(c.vsize, vsize);
        assert_eq!(c._field_of_view, field_of_view);
        assert_eq!(c.transform, Matrix::identity(4));
    }

    #[test]
    fn pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);

        assert!(c.pixel_size.approx_eq(0.01, Margin::default_f64()));
    }

    #[test]
    fn pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);

        assert!(c.pixel_size.approx_eq(0.01, Margin::default_f64()));
    }

    #[test]
    fn build_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);

        let r: Ray = c.ray_for_pixel(100, 50);
        assert_eq!(r.get_origin(), Tuple::new_point(0.0, 0.0, 0.0));
        assert_eq!(r.get_direction(), Tuple::new_vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn build_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);

        let r: Ray = c.ray_for_pixel(0, 0);
        assert_eq!(r.get_origin(), Tuple::new_point(0.0, 0.0, 0.0));

        assert_eq!(
            r.get_direction(),
            Tuple::new_vector(0.6651864261194508, 0.3325932130597254, -0.6685123582500481)
        );
    }

    #[test]
    fn build_a_ray_when_the_camera_is_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.transform =
            Transformation::rotation_y(PI / 4.0) * Transformation::translation(0.0, -2.0, 5.0);
        let r: Ray = c.ray_for_pixel(100, 50);
        assert_eq!(r.get_origin(), Tuple::new_point(0.0, 2.0, -5.0));

        assert_eq!(
            r.get_direction(),
            Tuple::new_vector(2.0_f64.sqrt() / 2.0, 0.0, -2.0_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let mut w = World::default();
        let mut c = Camera::new(11, 11, PI / 2.0);

        let from = Tuple::new_point(0.0, 0.0, -5.0);
        let to = Tuple::new_point(0.0, 0.0, 0.0);
        let up = Tuple::new_vector(0.0, 1.0, 0.0);

        c.transform = Transformation::view_transform(from, to, up);
        let image: Canvas = c.render(&mut w);

        assert_eq!(
            image.pixel_at(5, 5),
            Tuple::new_color(
                0.38066119308103435,
                0.47582649135129296,
                0.28549589481077575
            )
        );
    }
}
