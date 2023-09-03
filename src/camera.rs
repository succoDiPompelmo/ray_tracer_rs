use crate::matrices::Matrix;

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
    fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Camera {
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
}

#[cfg(test)]
mod tests {

    use std::f64::consts::PI;

    use float_cmp::{ApproxEq, F64Margin};

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
}
