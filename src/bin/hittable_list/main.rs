#![allow(dead_code, unused_imports)]
use core::f64;

use vec3::*;

use crate::common::{hittable, ray::Ray, sphere::Sphere, util::Interval};
#[path = "../../common/mod.rs"]
mod common;


fn ray_color(r: &Ray, world: &dyn hittable::Hittable) -> Color {
    let mut rec = hittable::HitRecord::new();
    if world.hit(r, Interval::new(0.0, f64::INFINITY), &mut rec) {
        return (rec.normal() + Color::from_float(1.0)) * 0.5;
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    let color1 = Color::from_float(1.0) * (1.0 - t);
    let color2 = Color::new(0.5, 0.7, 1.0) * t;
    color1 + color2
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i64 = 400;
    const IMAGE_HEIGHT: i64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i64;

    // World
    let mut world = hittable::HittableList::new();
    world.objects.push(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.objects.push(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::from_float(0.0);
    let horizontal = Vec3::<f64>::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::<f64>::new(0.0, viewport_height, 0.0);
    let depth = Vec3::<f64>::new(0.0, 0.0, focal_length);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - depth;

    // Render

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\r Scanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let u: f64 = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v: f64 = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let direction = lower_left_corner + horizontal * u + vertical * v - origin;
            let r = Ray::new(origin, direction);
            let pixel = ray_color(&r, &world);
            pixel.write_color();
        }
    }
}
