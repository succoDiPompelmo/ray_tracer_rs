mod canvas;
mod intersections;
mod lights;
mod matrices;
mod play;
mod rays;
mod spheres;
mod transformations;
mod tuples;

use canvas::Canvas;
use intersections::Intersection;
use play::Clock;
use rays::Ray;
use spheres::Sphere;

use crate::tuples::Tuple;

fn main() {
    let canvas_pixels = 100;
    let color = Tuple::new_color(1.0, 0.8, 0.6);
    let shape = Sphere::new();
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    let ray_origin = Tuple::new_point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;

    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    for y in 0..canvas_pixels - 1 {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels - 1 {
            let world_x = -half + pixel_size * x as f64;

            let position = Tuple::new_point(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(r);

            if let Some(_) = Intersection::hit(&xs) {
                canvas.write_pixel(color, x as isize, y as isize);
            }
        }
    }

    canvas.write_ppm_to_fs();
}

fn draw_cirlce() {
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
