#[macro_use]
extern crate impl_ops;
use std::ops;

use num::clamp;
use num_traits::{cast::FromPrimitive, float::Float};
use std::fmt;
// use std::ops::{Add, AddAssign};
use rand::distributions::{Distribution, Uniform};
use std::ops::{Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec3<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> From<(T, T, T)> for Vec3<T>
where
    T: Float,
{
    fn from(t: (T, T, T)) -> Vec3<T> {
        Vec3 {
            x: t.0,
            y: t.1,
            z: t.2,
        }
    }
}

pub type Point3 = Vec3<f64>;
pub type Color = Vec3<f64>;

impl<T> Vec3<T>
where
    T: Float + FromPrimitive,
{
    pub fn new(xvalue: T, yvalue: T, zvalue: T) -> Vec3<T> {
        Vec3 {
            x: xvalue,
            y: yvalue,
            z: zvalue,
        }
    }
    pub fn zero() -> Vec3<T> {
        Vec3 {
            x: T::from_f64(0.0).unwrap(),
            y: T::from_f64(0.0).unwrap(),
            z: T::from_f64(0.0).unwrap(),
        }
    }

    pub fn one() -> Vec3<T> {
        Vec3 {
            x: T::from_f64(1.0).unwrap(),
            y: T::from_f64(1.0).unwrap(),
            z: T::from_f64(1.0).unwrap(),
        }
    }

    pub fn from_float(value: T) -> Vec3<T> {
        Vec3 {
            x: value,
            y: value,
            z: value,
        }
    }

    pub fn length_squared(self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> T {
        let result = self.length_squared();
        result.sqrt()
    }

    #[inline]
    pub fn dot(&self, v: &Vec3<T>) -> T {
        self[0] * v[0] + self[1] * v[1] + self[2] * v[2]
    }

    #[inline]
    pub fn cross(u: &Vec<T>, v: &Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: u[1] * v[2] - u[2] * v[1],
            y: u[2] * v[0] - u[0] * v[2],
            z: u[0] * v[1] - u[1] * v[0],
        }
    }

    #[inline]
    pub fn unit_vector(self) -> Vec3<T> {
        return self / self.length();
    }

    pub fn write_color(&self) {
        // Write the translated [0,255] value of each color component.
        let ir: i32 = (255.999 * T::to_f64(&self.x).unwrap()) as i32;
        let ig: i32 = (255.999 * T::to_f64(&self.y).unwrap()) as i32;
        let ib: i32 = (255.999 * T::to_f64(&self.z).unwrap()) as i32;

        println!("{ir} {ig} {ib}");
    }

    pub fn write_color_aa(&self, samples_per_pixel: i64) {
        let scale = 1.0 / samples_per_pixel as f64;
        let r = T::to_f64(&self.x).unwrap() * scale;
        let g = T::to_f64(&self.y).unwrap() * scale;
        let b = T::to_f64(&self.z).unwrap() * scale;

        let ir = (256.0 * clamp(r, 0.0, 0.999)) as i32;
        let ig = (256.0 * clamp(g, 0.0, 0.999)) as i32;
        let ib = (256.0 * clamp(b, 0.0, 0.999)) as i32;

        println!("{ir} {ig} {ib}")
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let dist = Uniform::from(0.0..1.0);
        let randx = dist.sample(&mut rng);
        let randy = dist.sample(&mut rng);
        let randz = dist.sample(&mut rng);
        Vec3 {
            x: T::from(randx).unwrap(),
            y: T::from(randy).unwrap(),
            z: T::from(randz).unwrap(),
        }
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        let dist = Uniform::from(min..max);
        let randx = dist.sample(&mut rng);
        let randy = dist.sample(&mut rng);
        let randz = dist.sample(&mut rng);
        Vec3 {
            x: T::from(randx).unwrap(),
            y: T::from(randy).unwrap(),
            z: T::from(randz).unwrap(),
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        let mut point = Self::random_range(-1.0, 1.0);
        loop {
            if T::to_f64(&point.length_squared()).unwrap() < 1.0 {
                break;
            }
            point = Vec3::random_range(-1.0, 1.0);
        }
        return point;
    }
}

impl Vec3<f32> {
    pub const ZERO: Vec3<f32> = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub const ONE: Vec3<f32> = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
}

impl Vec3<f64> {
    pub const ZERO: Vec3<f64> = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub const ONE: Vec3<f64> = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
}

impl<T: Float + fmt::Display> fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<T: Float> Index<usize> for Vec3<T> {
    type Output = T;
    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!(
                "Vec3 index must be 0, 1, or 2 corresponding to x, y, or z, got {}.",
                i,
            ),
        }
    }
}

impl<T: Float> IndexMut<usize> for Vec3<T> {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut T {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!(
                "Vec3 index must be 0, 1, or 2 corresponding to x, y, or z, got {}.",
                i,
            ),
        }
    }
}

// impl<T: Float> Add for Vec3<T> {
//     type Output = Vec3<T>;

//     #[inline]
//     fn add(self, other: Vec3<T>) -> Vec3<T> {
//         Vec3 {
//             x: self.x + other.x,
//             y: self.y + other.y,
//             z: self.z + other.z,
//         }
//     }
// }

// impl<T: Float> Add<T> for Vec3<T> {
//     type Output = Vec3<T>;

//     #[inline]
//     fn add(self, other: T) -> Vec3<T> {
//         Vec3 {
//             x: self.x + other,
//             y: self.y + other,
//             z: self.z + other,
//         }
//     }
// }
// struct MyType<T: Float>(T);
// impl<T: Float> Add<Vec3<T>> for MyType<T> {
//     type Output = Vec3<T>;
//     #[inline]
//     fn add(self, other: Vec3<T>) -> Vec3<T> {
//         other + self.0
//     }
// }

// impl<T: Float> AddAssign for Vec3<T> {
//     #[inline]
//     fn add_assign(&mut self, other: Vec3<T>) {
//         *self = *self + other;
//     }
// }

// impl<T: Float> AddAssign<T> for Vec3<T> {
//     #[inline]
//     fn add_assign(&mut self, other: T) {
//         *self = *self + other;
//     }
// }

impl_op_ex!(+ |a: &Vec3<f32>, b: &Vec3<f32>| -> Vec3<f32> {Vec3{x: a.x + b.x, y: a.y + b.y, z: a.z + b.z}});
impl_op_ex!(+ |a: &Vec3<f64>, b: &Vec3<f64>| -> Vec3<f64> {Vec3{x: a.x + b.x, y: a.y + b.y, z: a.z + b.z}});
impl_op_ex_commutative!(+ |a: &Vec3<f32>, b: f32| -> Vec3<f32> { Vec3{x: a.x + b, y: a.y + b, z: a.z + b}});
impl_op_ex_commutative!(+ |a: &Vec3<f64>, b: f64| -> Vec3<f64> { Vec3{x: a.x + b, y: a.y + b, z: a.z + b}});
impl_op_ex!(+= |a: &mut Vec3<f32>, b: Vec3<f32>| { *a = *a + b});
impl_op_ex!(+= |a: &mut Vec3<f64>, b: Vec3<f64>| { *a = *a + b });
impl_op_ex!(+= |a: &mut Vec3<f32>, b: f32| { *a = *a + b });
impl_op_ex!(+= |a: &mut Vec3<f64>, b: f64| { *a = *a + b });

impl<T: Float> Sub for Vec3<T> {
    type Output = Vec3<T>;

    #[inline]
    fn sub(self, other: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Float> Sub<T> for Vec3<T> {
    type Output = Vec3<T>;

    #[inline]
    fn sub(self, other: T) -> Vec3<T> {
        Vec3 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl<T: Float> SubAssign for Vec3<T> {
    #[inline]
    fn sub_assign(&mut self, other: Vec3<T>) {
        *self = *self - other;
    }
}

impl<T: Float> SubAssign<T> for Vec3<T> {
    #[inline]
    fn sub_assign(&mut self, other: T) {
        *self = *self - other;
    }
}

impl<T: Float> Mul for Vec3<T> {
    type Output = Vec3<T>;

    #[inline]
    fn mul(self, other: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T: Float> Mul<T> for Vec3<T> {
    type Output = Vec3<T>;

    #[inline]
    fn mul(self, other: T) -> Vec3<T> {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<T: Float> MulAssign for Vec3<T> {
    #[inline]
    fn mul_assign(&mut self, other: Vec3<T>) {
        *self = *self * other;
    }
}

impl<T: Float> MulAssign<T> for Vec3<T> {
    #[inline]
    fn mul_assign(&mut self, other: T) {
        *self = *self * other;
    }
}

impl<T: Float> Div for Vec3<T> {
    type Output = Vec3<T>;

    #[inline]
    fn div(self, other: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl<T: Float> Div<T> for Vec3<T> {
    type Output = Vec3<T>;

    #[inline]
    fn div(self, other: T) -> Vec3<T> {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl<T: Float> DivAssign for Vec3<T> {
    #[inline]
    fn div_assign(&mut self, other: Vec3<T>) {
        *self = *self / other;
    }
}

impl<T: Float> DivAssign<T> for Vec3<T> {
    #[inline]
    fn div_assign(&mut self, other: T) {
        *self = *self / other;
    }
}

impl<T: Float> Neg for Vec3<T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[test]
fn indexing() {
    let x: Vec3<f64> = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    assert_eq!(x[0], 1.0);
    assert_eq!(x[1], 2.0);

    let mut y: Vec3<f64> = Vec3 {
        x: 10.0,
        y: 2.0,
        z: 3.0,
    };
    y[0] = 1.0;
    assert_eq!(y, x);
}

#[test]
#[should_panic(expected = "Vec3 index must be 0, 1, or 2 corresponding to x, y, or z, got 5.")]
fn invalid_indexing() {
    let x: Vec3<f64> = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    x[5];
}

#[test]
fn addition() {
    let x: Vec3<f64> = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 2.0,
    };
    let y: Vec3<f64> = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 2.0,
    };
    let z: Vec3<f64> = Vec3 {
        x: 0.0,
        y: 2.0,
        z: 4.0,
    };
    let x_plus_one: Vec3<f64> = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    // Test Vec3 + Vec3
    assert_eq!(z, x + y);
    // Test Vec3 + float
    assert_eq!(x_plus_one, x + 1.0);
    // Test float + Vec3
    assert_eq!(x_plus_one, 1.0 + x);

    let mut mx: Vec3<f64> = x;
    let mut my: Vec3<f64> = y;

    // Test += operator
    mx += my;
    assert_eq!(mx, z);

    // Ensure commutativity still works
    mx = x;
    my += mx;
    assert_eq!(my, z);
}

#[test]
fn subtraction() {
    let x: Vec3<f64> = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 2.0,
    };
    let y: Vec3<f64> = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 2.0,
    };
    let x_minus_one: Vec3<f64> = Vec3 {
        x: -1.0,
        y: 0.0,
        z: 1.0,
    };
    assert_eq!(Vec3::<f64>::ZERO, x - y);
    assert_eq!(x_minus_one, x - 1.0);
}

#[test]
fn length() {
    let x: Vec3<f64> = Vec3 {
        x: 2.0,
        y: 3.0,
        z: 6.0,
    };
    let y = 7.0;
    assert_eq!(x.length(), y);
    assert_eq!(Vec3::length(x), y);
}
