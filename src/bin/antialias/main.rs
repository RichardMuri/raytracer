use vec3::*;

use crate::common::{camera::Camera, hittable::HittableList, sphere::Sphere};
#[path ="../../common/mod.rs"]
mod common;

fn main() {
    // World
    let mut world = HittableList::new();
    world.objects.push(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.objects.push(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    // Camera
    let aspect_ratio: f64 = 16.0/ 9.0;
    let image_width: i64 = 400;
    let samples_per_pixel: i64 = 100;
    let mut cam = Camera::new(aspect_ratio, image_width, samples_per_pixel);

    cam.render(&world);
}
