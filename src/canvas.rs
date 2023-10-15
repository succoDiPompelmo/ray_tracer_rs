use std::{fs::File, io::Write};

use crate::tuples::Tuple;

pub struct Canvas {
    width: usize,
    height: usize,
    // TODO: This has bad data locality since the column vectors could be scattered
    // accross the heap. Some library to better handle this could already exists. Is needed
    // to evaluate the alternatives. https://www.reddit.com/r/rust/comments/nfoi4j/how_can_i_create_a_2d_array/
    state: Vec<Vec<Tuple>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let state = vec![vec![Tuple::black(); width]; height];
        Canvas {
            width,
            height,
            state,
        }
    }

    #[cfg(test)]
    pub fn pixel_at(&self, x: usize, y: usize) -> Tuple {
        self.state[y][x]
    }

    pub fn write_pixel(&mut self, color: Tuple, x: isize, y: isize) {
        if y < self.height as isize && y >= 0 && x < self.width as isize && x >= 0 {
            self.state[y as usize][x as usize] = color
        }
    }

    fn to_ppm(&self) -> String {
        let magic_number = "P3".to_owned();
        let size = format!("{} {}", self.width, self.height);
        let max_color_value = "255".to_owned();

        let mut rows: Vec<String> = vec![];

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
                .collect::<Vec<String>>()
                .join(" ");

            let mut space_cnt = 0;
            let mut output = vec![];
            for el in format_row.chars() {
                if el.is_ascii_whitespace() {
                    space_cnt += 1;
                }

                if space_cnt % 10 == 0 && el.is_ascii_whitespace() {
                    output.push('\n');
                } else {
                    output.push(el);
                }
            }

            rows.push(output.into_iter().collect());
        }
        let body = rows.join("\n");

        format!(
            "{}\n{}\n{}\n{}\n",
            magic_number, size, max_color_value, body
        )
    }

    pub fn write_ppm_to_fs(&self) {
        let ppm = self.to_ppm();
        let mut file = File::create("foo.ppm").unwrap();
        file.write_all(ppm.as_bytes()).unwrap();
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

        assert_eq!(canvas.width, width);
        assert_eq!(canvas.height, height);

        for x in 0..height {
            for y in 0..width {
                assert_eq!(canvas.pixel_at(y, x), Tuple::black());
            }
        }
    }

    #[test]
    fn write_a_pixel() {
        let color = Tuple::new_color(1.0, 0.0, 0.0);
        let mut canvas = Canvas::new(10, 20);

        canvas.write_pixel(color, 2, 3);

        assert_eq!(canvas.pixel_at(2, 3), color);
    }

    #[test]
    fn canvas_to_ppm() {
        let mut canvas = Canvas::new(3, 3);
        canvas.write_pixel(Tuple::new_color(1.5, 0.0, 0.0), 0, 0);
        canvas.write_pixel(Tuple::new_color(0.0, 0.5, 0.0), 1, 1);
        canvas.write_pixel(Tuple::new_color(-0.5, 0.0, 1.0), 2, 2);
        let ppm = canvas.to_ppm();

        let expected = "\
        P3\n\
        3 3\n\
        255\n\
        255 0 0 0 0 0 0 0 0\n\
        0 0 0 0 128 0 0 0 0\n\
        0 0 0 0 0 0 0 0 255\n";

        assert_eq!(ppm, expected);
    }

    #[test]
    fn canvas_with_high_width_to_ppm() {
        let mut canvas = Canvas::new(8, 3);
        let color = Tuple::new_color(1.0, 0.8, 0.6);
        for x in 0..canvas.height {
            for y in 0..canvas.width {
                canvas.write_pixel(color, y as isize, x as isize);
            }
        }
        let ppm = canvas.to_ppm();
        let expected = "\
        P3\n\
        8 3\n\
        255\n\
        255 204 153 255 204 153 255 204 153 255\n\
        204 153 255 204 153 255 204 153 255 204\n\
        153 255 204 153\n\
        255 204 153 255 204 153 255 204 153 255\n\
        204 153 255 204 153 255 204 153 255 204\n\
        153 255 204 153\n\
        255 204 153 255 204 153 255 204 153 255\n\
        204 153 255 204 153 255 204 153 255 204\n\
        153 255 204 153\n";

        assert_eq!(ppm, expected);
    }
}
