use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{canvas::Canvas, matrices::Matrix, rays::Ray, tuples::Tuple, world::World};

pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f64,
    transform: Matrix,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Camera {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = (hsize as f64 / vsize as f64);

        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = (half_width * 2.0) / hsize as f64;

        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::identity(4),
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

        // Remember that canvas is at z = -1
        let pixel = self.transform.invert() * Tuple::new_point(world_x, world_y, -1.0);
        let origin = self.transform.invert() * Tuple::new_point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);
        let mut pixels = vec![];

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                pixels.push((x, y));
            }
        }

        let colors: Vec<(usize, usize, Tuple)> = pixels
            .par_iter()
            .map(move |(x, y)| {
                let ray = self.ray_for_pixel(*x, *y);
                (*x, *y, world.color_at(&ray))
            })
            .collect();

        for (x, y, color) in colors {
            image.write_pixel(color, x as isize, y as isize);
        }

        image
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }
}

#[cfg(test)]
mod tests {

    use std::f64::consts::PI;

    use float_cmp::{ApproxEq, F64Margin};

    use crate::{canvas::Canvas, transformations::Transformation, tuples::Tuple, world::World};

    use super::*;

    #[test]
    fn construct_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;

        let c = Camera::new(hsize, vsize, field_of_view);

        assert!(c.hsize == hsize);
        assert!(c.vsize == vsize);
        assert!(c.field_of_view == field_of_view);
        assert!(c.transform == Matrix::identity(4));
    }

    #[test]
    fn pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);

        let margin = F64Margin {
            ulps: 2,
            epsilon: 1e-14,
        };

        assert!(c.pixel_size.approx_eq(0.01, margin));
    }

    #[test]
    fn pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);

        let margin = F64Margin {
            ulps: 2,
            epsilon: 1e-14,
        };

        assert!(c.pixel_size.approx_eq(0.01, margin));
    }

    #[test]
    fn build_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);

        let r: Ray = c.ray_for_pixel(100, 50);
        assert!(r.get_origin() == Tuple::new_point(0.0, 0.0, 0.0));
        assert!(r.get_direction() == Tuple::new_vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn build_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);

        let r: Ray = c.ray_for_pixel(0, 0);
        assert!(r.get_origin() == Tuple::new_point(0.0, 0.0, 0.0));

        assert!(
            r.get_direction()
                == Tuple::new_vector(0.6651864261194508, 0.3325932130597254, -0.6685123582500481)
        );
    }

    #[test]
    fn build_a_ray_when_the_camera_is_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.transform =
            Transformation::rotation_y(PI / 4.0) * Transformation::translation(0.0, -2.0, 5.0);
        let r: Ray = c.ray_for_pixel(100, 50);
        assert!(r.get_origin() == Tuple::new_point(0.0, 2.0, -5.0));

        assert!(
            r.get_direction()
                == Tuple::new_vector(2.0_f64.sqrt() / 2.0, 0.0, -2.0_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = World::default();
        let mut c = Camera::new(11, 11, PI / 2.0);

        let from = Tuple::new_point(0.0, 0.0, -5.0);
        let to = Tuple::new_point(0.0, 0.0, 0.0);
        let up = Tuple::new_vector(0.0, 1.0, 0.0);

        c.transform = Transformation::view_transform(from, to, up);
        let image: Canvas = c.render(w);

        assert!(
            image.pixel_at(5, 5)
                == Tuple::new_color(
                    0.38066119308103435,
                    0.47582649135129296,
                    0.28549589481077575
                )
        );
    }
}
