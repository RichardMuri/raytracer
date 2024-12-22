use rand::distributions::{Distribution, Uniform};

use ray::Ray;
use vec3::*;

use crate::{camera::Camera, hittable::HittableList};
#[path = "../../hittable.rs"]
mod hittable;
#[path = "../../ray.rs"]
mod ray;

#[path = "../../sphere.rs"]
mod sphere;

#[path = "../../camera.rs"]
mod camera;

fn ray_color(r: &ray::Ray, world: &dyn hittable::Hittable, depth: i64) -> Color {
    let mut rec = hittable::HitRecord::new();

    if depth <= 0 {
        return Color::zero();
    }

    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        let target = rec.p + rec.normal() + Point3::random_in_unit_sphere();
        return ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1) * 0.5;
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    let color1 = Color::from_float(1.0) * (1.0 - t);
    let color2 = Color::new(0.5, 0.7, 1.0) * t;
    color1 + color2
}

fn main() {
    let mut rng = rand::thread_rng();
    let dist = Uniform::from(0.0..1.0);
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i64 = 400;
    const IMAGE_HEIGHT: i64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i64;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i64 = 50;

    // World
    let mut world = HittableList::new();
    world.objects.push(Box::new(sphere::Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.objects.push(Box::new(sphere::Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    // Camera
    let cam = Camera::default();

    // Render

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\r Scanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let mut pixel = Color::from_float(0.0);
            for _s in 0..SAMPLES_PER_PIXEL {
                let randu = dist.sample(&mut rng);
                let u: f64 = (i as f64 + randu) / (IMAGE_WIDTH - 1) as f64;
                let randv = dist.sample(&mut rng);
                let v: f64 = (j as f64 + randv) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);

                pixel += ray_color(&r, &world, MAX_DEPTH);
            }

            pixel.write_color_aa(SAMPLES_PER_PIXEL.into());
        }
    }
}
