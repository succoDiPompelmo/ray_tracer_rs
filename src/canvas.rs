use image::{ImageBuffer, Rgb, RgbImage};

use crate::tuples::Tuple;

const OUTPUT_DIR: &str = "output";

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

    pub fn save(&self, filename: String) {
        let mut img: RgbImage = ImageBuffer::new(self.width as u32, self.height as u32);
        for x in 0..self.height {
            for y in 0..self.width {
                let pixel = self.state[x][y];
                img.put_pixel(y as u32, x as u32, Rgb(Canvas::format_pixel(pixel)))
            }
        }
        img.save(format!("{OUTPUT_DIR}/{filename}")).unwrap();
    }

    fn format_pixel(pixel: Tuple) -> [u8; 3] {
        let x = ((pixel.x * 255.0).round() as u8).clamp(0, 255);
        let y = ((pixel.y * 255.0).round() as u8).clamp(0, 255);
        let z = ((pixel.z * 255.0).round() as u8).clamp(0, 255);

        return [x, y, z];
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
}
