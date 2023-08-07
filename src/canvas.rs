use crate::tuples::Tuple;

struct Canvas {
    width: usize,
    height: usize,
    // TODO: This has bad data locality since the column vectors could be scattered
    // accross the heap. Some library to better handle this could already exists. Is needed
    // to evaluate the alternatives. https://www.reddit.com/r/rust/comments/nfoi4j/how_can_i_create_a_2d_array/
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
        self.state[y][x]
    }

    fn write_pixel(&mut self, color: Tuple, x: usize, y: usize) {
        self.state[y][x] = color
    }

    fn to_ppm(&self) -> String {
        let magic_number = "P3".to_owned();
        let size = format!("{} {}", self.width, self.height);
        let max_color_value = "255".to_owned();

        let mut rows = vec![];

        for x in 0..self.height {
            let mut row = vec![];

            for y in 0..self.width {
                let pixel = self.state[x][y];
                row.push(pixel.x);
                row.push(pixel.y);
                row.push(pixel.z);
            }

            let format_row = row
                .iter()
                .map(|el| el * 255.0)
                .map(|el| el.round() as usize)
                .map(|el| el.clamp(0, 255))
                .map(|el| el.to_string())
                .collect::<Vec<String>>();

            rows.push(format_row.join(" "));
        }
        let body = rows.join("\n");

        format!("{}\n{}\n{}\n{}", magic_number, size, max_color_value, body)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn creating_a_canvas() {
        let width = 10;
        let height = 20;

        let canvas = Canvas::new(width, height);

        assert!(canvas.width() == width);
        assert!(canvas.height() == height);

        for x in 0..height {
            for y in 0..width {
                assert!(canvas.pixel_at(y, x) == Tuple::new_color(0.0, 0.0, 0.0));
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

    #[test]
    fn canvas_to_ppm() {
        let mut canvas = Canvas::new(5, 3);
        canvas.write_pixel(Tuple::new_color(1.5, 0.0, 0.0), 0, 0);
        canvas.write_pixel(Tuple::new_color(0.0, 0.5, 0.0), 2, 1);
        canvas.write_pixel(Tuple::new_color(-0.5, 0.0, 1.0), 4, 2);
        let ppm = canvas.to_ppm();

        let expected = "\
        P3\n\
        5 3\n\
        255\n\
        255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n\
        0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n\
        0 0 0 0 0 0 0 0 0 0 0 0 0 0 255";

        assert!(ppm == expected);
    }
}