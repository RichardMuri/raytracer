use crate::common::hittable::{Hittable, HitRecord};
use crate::common::util::Interval;
use crate::common::ray::*;
use vec3::*;
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(c: Point3, r: f64) -> Self {
        Sphere {
            center: c,
            radius: r,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(&r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        };
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;

            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.point() - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        true
    }
}
