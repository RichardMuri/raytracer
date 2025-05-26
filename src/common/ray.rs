use vec3::*;

pub struct Ray {
    orig: Point3,
    dir: Direction,
}

impl Ray {
    pub fn new(p: Point3, d: Direction) -> Ray {
        Ray { orig: p, dir: d }
    }
    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Direction {
        self.dir
    }

    pub fn at(&self, t: f64) -> Vec3<f64> {
        self.orig + self.dir * t
    }
}
