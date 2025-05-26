use crate::common::{hittable, ray::Ray, util::Interval};
use vec3::*;

pub struct Camera {
    // origin: Point3,
    // lower_left_corner: Point3,
    // horizontal: Vec3<f64>,
    // vertical: Vec3<f64>,
    // Public fields
    pub aspect_ratio: f64,
    pub image_width: i64,
    pub samples_per_pixel: i64,

    // Private fields
    image_height: i64,
    center: Point3,
    pixel100_location: Point3,
    pixel_delta_u: Vec3<f64>,
    pixel_delta_v: Vec3<f64>,
}

impl Default for Camera {
    fn default() -> Self {
        // let aspect_ratio = 16.0 / 9.0;
        // let viewport_height = 2.0;
        // let focal_length = 1.0;

        // let origin = Point3::zero();

        let aspect_ratio: f64 = 1.0;
        let image_width : i64 = 100;
        let samples_per_pixel: i64 = 10;

        return Self::new(aspect_ratio, image_width, samples_per_pixel);
    }
}

impl Camera {
    // pub fn new(aspect_ratio: f64, viewport_height: f64, focal_length: f64, origin: Point3) -> Self {
    //     let viewport_width = aspect_ratio * viewport_height;
    //     let horizontal = Vec3::<f64> {
    //         x: viewport_width,
    //         y: 0.0,
    //         z: 0.0,
    //     };
    //     let vertical = Vec3::<f64> {
    //         x: 0.0,
    //         y: viewport_height,
    //         z: 0.0,
    //     };
    //     let distance = Vec3::<f64> {
    //         x: 0.0,
    //         y: 0.0,
    //         z: focal_length,
    //     };
    //     let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - distance;

    //     Camera {
    //         origin: origin,
    //         lower_left_corner: lower_left_corner,
    //         horizontal: horizontal,
    //         vertical: vertical,
    //     }
    // }

    pub fn new(aspect_ratio: f64, image_width: i64, samples_per_pixel: i64) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as i64;
        let center = Point3::zero();
        let pixel100_location = Point3::zero();
        let pixel_delta_u = Vec3::zero();
        let pixel_delta_v = Vec3::zero();

        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            image_height,
            center,
            pixel100_location,
            pixel_delta_u,
            pixel_delta_v,
        }

    }

    pub fn render(&mut self, world: &dyn hittable::Hittable)
    {
        self.initialize();

        println!("P3\n{0} {1}\n255", self.image_width, self.image_height);

        for j in (0..self.image_height) {
            eprint!("\r Scanlines remaining: {j}");
            for i in 0..self.image_width {
                let mut pixel_color = Color::zero();
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Camera::ray_color(&r, world);
                }
                pixel_color.write_color_aa(self.samples_per_pixel);
            }
        }

        eprintln!("\rDone rendering!");
    }

    pub fn get_ray(&self, i: i64, j: i64) -> Ray {
        // Construct a camera ray originating from the origin and directed at
        // randomly sampled point around the pixel location (i, j)

        let offset = Vec3::<f64>::sample_square();
        let pixel_sample = self.pixel100_location +
            (self.pixel_delta_u * (i as f64 + offset.x)) +
            (self.pixel_delta_v * (j as f64 + offset.y));

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn initialize(&mut self)
    {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i64;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.center = Point3::zero();

        // Determine viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel
        let viewport_upper_left = self.center - Vec3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        self.pixel100_location = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }

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
}
