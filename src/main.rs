mod canvas;
mod play;
mod tuples;

use canvas::Canvas;

use crate::tuples::Tuple;

fn main() {
    let mut canvas = Canvas::new(900, 500);

    let color = Tuple::new_color(1.0, 0.8, 0.6);

    let env = play::Environment::new(
        Tuple::new_vector(0.0, -0.1, 0.0),
        Tuple::new_vector(-0.01, 0.0, 0.0),
    );

    let mut proj = play::Projectile::new(
        Tuple::new_point(0.0, 1.0, 0.0),
        Tuple::new_vector(100.0, 150.0, 0.0).normalize() * 10.0,
    );

    for _ in 0..10000 {
        proj = env.tick(proj);

        if proj.get_y() < 0.0 {
            break;
        }

        canvas.write_pixel(color, proj.get_x() as usize, canvas.height() - (proj.get_y() as usize));
    }

    canvas.write_ppm_to_fs();
}
