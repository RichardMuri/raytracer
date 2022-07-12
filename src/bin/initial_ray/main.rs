use vec3::*;
#[path = "../../ray.rs"]
mod ray;

fn hit_sphere(center: &Point3, radius: f64, r: &ray::Ray) -> f64 {
    let oc = r.origin() - *center;
    let a = r.direction().length_squared();
    let half_b = oc.dot(&r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(r: &ray::Ray) -> Color {
    let t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = (r.at(t) - Vec3::<f64>::new(0.0, 0.0, -1.0)).unit_vector();
        return (n + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    let color1 = Color::new(1.0, 1.0, 1.0) * (1.0 - t);
    let color2 = Color::new(0.5, 0.7, 1.0) * t;
    color1 + color2
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i64 = 400;
    const IMAGE_HEIGHT: i64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i64;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
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
            let r = ray::Ray::new(origin, direction);
            let pixel = ray_color(&r);
            pixel.write_color();
        }
    }
}
