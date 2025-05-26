use crate::common::{ray::Ray, util::Interval};
use vec3::*;

#[derive(Clone, Copy, Default)]
pub struct HitRecord {
    front_face: bool,
    normal: Vec3<f64>,
    pub p: Point3,
    pub t: f64,
}

impl HitRecord {
    #[inline]
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3<f64>) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        };
    }

    pub fn new() -> Self {
        HitRecord {
            p: Point3::from_float(0.0),
            normal: Vec3::<f64>::from_float(0.0),
            t: 0.0,
            front_face: true,
        }
    }

    pub fn normal(self) -> Vec3<f64> {
        self.normal
    }

    pub fn point(self) -> Vec3<f64> {
        self.p
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}
