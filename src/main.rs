mod canvas;
mod play;
mod tuples;

use crate::tuples::Tuple;

fn main() {
    let env = play::Environment::new(
        Tuple::new_vector(0.0, -0.1, 0.0),
        Tuple::new_vector(-0.01, 0.0, 0.0),
    );

    let mut proj = play::Projectile::new(
        Tuple::new_point(0.0, 1.0, 0.0),
        Tuple::new_vector(1.0, 1.0, 0.0).normalize(),
    );

    for _ in 0..100 {
        proj = env.tick(proj);

        if proj.get_y() < 0.0 {
            break;
        }

        println!("{:?}", proj);
    }
}
