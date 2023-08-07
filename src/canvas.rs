use crate::tuples::Tuple;

struct Canvas {
    width: usize,
    height: usize,
    // TODO: This has bad data locality since the column vectors could be scattered
    // accross the heap. Some library to better handle this could already exists. Is needed
    // to evaluate the alternatives.
    state: Vec<Vec<Tuple>>,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Canvas {
        let state = vec![vec![Tuple::new_color(0.0, 0.0, 0.0); width]; height];
        Canvas {
            width,
            height,
            state,
        }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn pixel_at(&self, x: usize, y: usize) -> Tuple {
        self.state[x][y]
    }

    fn write_pixel(&mut self, color: Tuple, x: usize, y: usize) {
        self.state[x][y] = color
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn creating_a_canvas() {
        let width = 10;
        let height = 10;

        let canvas = Canvas::new(width, height);

        assert!(canvas.width() == width);
        assert!(canvas.height() == height);

        for x in 0..width {
            for y in 0..height {
                assert!(canvas.pixel_at(x, y) == Tuple::new_color(0.0, 0.0, 0.0));
            }
        }
    }

    #[test]
    fn write_a_pixel() {
        let color = Tuple::new_color(1.0, 0.0, 0.0);
        let mut canvas = Canvas::new(10, 20);

        canvas.write_pixel(color, 2, 3);

        assert!(canvas.pixel_at(2, 3) == color);
    }
}
