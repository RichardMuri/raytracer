use crate::ray::Ray;
use vec3::*;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3<f64>,
    vertical: Vec3<f64>,
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let focal_length = 1.0;

        let origin = Point3::zero();

        return Self::new(aspect_ratio, viewport_height, focal_length, origin);
    }
}

impl Camera {
    pub fn new(aspect_ratio: f64, viewport_height: f64, focal_length: f64, origin: Point3) -> Self {
        let viewport_width = aspect_ratio * viewport_height;
        let horizontal = Vec3::<f64> {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };
        let vertical = Vec3::<f64> {
            x: 0.0,
            y: viewport_height,
            z: 0.0,
        };
        let distance = Vec3::<f64> {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - distance;

        Camera {
            origin: origin,
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction =
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin;
        return Ray::new(self.origin, direction);
    }
}
