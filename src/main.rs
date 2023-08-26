mod canvas;
mod intersections;
mod matrices;
mod play;
mod rays;
mod spheres;
mod transformations;
mod tuples;

use canvas::Canvas;
use play::Clock;

use crate::tuples::Tuple;

fn main() {
    let mut canvas = Canvas::new(900, 500);

    let ticks = 300;

    let color = Tuple::new_color(1.0, 0.8, 0.6);
    let mut clock = Clock::new(100.0, ticks);

    for _ in 0..ticks {
        canvas.write_pixel(
            color,
            (clock.get_x() + 450.0) as isize,
            (clock.get_y() + 250.0) as isize,
        );
        clock = clock.tick();
    }

    canvas.write_ppm_to_fs();
}

fn draw_projectile() {
    let mut canvas = Canvas::new(900, 500);

    let color = Tuple::new_color(1.0, 0.8, 0.6);

    let env = play::Environment::new(
        Tuple::new_vector(0.0, -0.1, 0.0),
        Tuple::new_vector(-0.01, 0.0, 0.0),
    );

    let mut proj = play::Projectile::new(
        Tuple::new_point(0.0, 1.0, 0.0),
        Tuple::new_vector(100.0, 150.0, 0.0).normalize() * 15.0,
    );

    for _ in 0..10000 {
        proj = env.tick(proj);

        if proj.get_y() < 0.0 {
            break;
        }

        canvas.write_pixel(
            color,
            proj.get_x() as isize,
            canvas.height() as isize - (proj.get_y() as isize),
        );
    }

    canvas.write_ppm_to_fs();
}
