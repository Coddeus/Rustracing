use std::{f64::EPSILON, ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}};

use crate::utils::rand_within;

pub type Point3 = Vec3;
pub type Color3 = Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn rand(min: f64, max: f64) -> Self {
        Vec3::new(rand_within(min, max), rand_within(min, max), rand_within(min, max))
    }
    /// Returns a random unit vector on the with the same direction as the given normal vector.
    pub fn rand_unit() -> Self {
        loop {
            let p = Vec3::rand(-1., 1.);
            if p.len_squared() < 1. {
                return p.unit()
            }
        }
    }
    /// Returns a random unit vector on the with the same direction as the given normal vector (= on the right hemisphere).
    pub fn rand_unit_onhem(normal: Vec3) -> Self {
        loop {
            let p = Vec3::rand(-1., 1.);
            if p.len_squared() < 1. {
                let p = p.unit();
                return
                    if p * normal > 0. { p }
                    else { -p }
            }
        }
    }

    pub fn near_zero(&self) -> bool {
        self.x.abs() < EPSILON && self.y.abs() < EPSILON && self.z.abs() < EPSILON
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn negate(&mut self) {
        *self = -*self;
    }
    /// Returns the multiplications of each value of `self` with the corresponding value of `rhs`.
    pub fn mult(self, rhs: Self) -> Self {
         Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
         }
    }
    /// Multiplies each value of `self` with the corresponding value of `rhs`.
    pub fn multeq(&mut self, rhs: Self)  {
         *self = Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
         }
    }
    
    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }
    pub fn len_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn unit(&self) -> Vec3 {
        let len = self.len();
        *self / len
    }
    pub fn normalize(&mut self) {
        let len = self.len();
        *self = *self / len;
    }

    /// Clamps all values to the interval [min; max]
    pub fn clamp(&mut self, min: f64, max: f64) {
        self.x = self.x.clamp(min, max);
        self.y = self.y.clamp(min, max);
        self.z = self.z.clamp(min, max);
    }

    pub fn refract(&self, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = ((-*self) * n).min(1.0);
        let r_out_perp: Vec3 = etai_over_etat * (*self + cos_theta * n);
        let r_out_parallel: Vec3 = - (1.0 - r_out_perp.len_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }
}

impl Color3 {
    /// The string to be written in order to represent this color in a ppm file
    /// Expects normalized float color values
    pub fn ppm(&self) -> String {
        format!("{} {} {}\n", (self.x * 255.999).floor() as u32, (self.y * 255.999).floor() as u32, (self.z * 255.999).floor() as u32)
    }

    /// Converts this color to from linear to gamma 2.
    pub fn to_gamma_2(&mut self) {
        self.x = self.x.sqrt();
        self.y = self.y.sqrt();
        self.z = self.z.sqrt();
    }
}

// ------------------------- General

impl Default for Vec3 {
    fn default() -> Self {
        Self::new(0., 0., 0.)
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

// ------------------------- Vec3s chaining

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

// ------------------------- Vec3s Negation

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// ------------------------- Vec3s scaling

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}
impl Div<Vec3> for f64 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

// ------------------------- Dot product

impl Mul<Vec3> for Vec3 {
    type Output = f64;
    fn mul(self, rhs: Vec3) -> f64 {
        self.x*rhs.x + self.y*rhs.y + self.z*rhs.z
    }
}

// ------------------------- Cross product

impl BitAnd for Vec3 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}
impl BitAndAssign for Vec3 {
    fn bitand_assign(&mut self, rhs: Self) {
        let orig_x = self.x;
        let orig_y = self.y;
        let orig_z = self.z;

        self.x = orig_y * rhs.z - orig_z * rhs.y;
        self.y = orig_z * rhs.x - orig_x * rhs.z;
        self.z = orig_x * rhs.y - orig_y * rhs.x;
    }
}

// ------------------------- Reflection: assert!(incident | normal == reflected)

impl BitOr for Vec3 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        self - 2. * (self * rhs) * rhs
    }
}
impl BitOrAssign for Vec3 {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self - 2. * (*self * rhs) * rhs;
    }
}


// -------------------------------------------------- Tests


#[test]
fn types() {
    let v = Vec3::new(0., 1., 2.);
    let p = Point3::new(0., 1., 2.);
    let c = Color3::new(0., 1., 2.);

    assert!(v == p && p == c)
}
#[test]
fn add() {
    let t = Vec3::new(6., 8., 2.);
    let u = Vec3::new(3., 7., 2.);
    let v = Vec3::new(3., 1., 0.);

    assert!(t == u + v)
}
#[test]
fn addeq() {
    let t = Vec3::new(6., 8., 2.);
    let mut u = Vec3::new(3., 7., 2.);
    let v = Vec3::new(3., 1., 0.);

    u += v;

    assert!(t == u)
}
#[test]
fn sub() {
    let t = Vec3::new(6., 8., 2.);
    let u = Vec3::new(3., 7., 2.);
    let v = Vec3::new(3., 1., 0.);

    assert!(t - u == v)
}
#[test]
fn subeq() {
    let mut t = Vec3::new(6., 8., 2.);
    let u = Vec3::new(3., 7., 2.);
    let v = Vec3::new(3., 1., 0.);

    t -= u;

    assert!(t == v)
}
#[test]
fn mul() {
    let u = Vec3::new(3., 7., 2.);
    let v = Vec3::new(12., 28., 8.);

    assert!(u * 4. == v)
}
#[test]
fn muleq() {
    let mut u = Vec3::new(3., 7., 2.);
    let v = Vec3::new(12., 28., 8.);

    u *= 4.;

    assert!(u == v)
}
#[test]
fn div() {
    let u = Vec3::new(3., 7., 2.);
    let v = Vec3::new(12., 28., 8.);

    assert!(v / 4. == u)
}
#[test]
fn diveq() {
    let u = Vec3::new(3., 7., 2.);
    let mut v = Vec3::new(12., 28., 8.);

    v /= 4.;

    assert!(v == u)
}
#[test]
fn dot() {
    let u = Vec3::new(2., 3., 1.);
    let v = Vec3::new(-1., 4., 2.);

    assert!(u * v == 12.)
}
#[test]
fn cross() {
    let t = Vec3::new(2., 3., 1.);
    let u = Vec3::new(-1., 4., 2.);
    let v = Vec3::new(2., -5., 11.);

    assert!(t & u == v)
}
#[test]
fn crosseq() {
    let mut t = Vec3::new(2., 3., 1.);
    let u = Vec3::new(-1., 4., 2.);
    let v = Vec3::new(2., -5., 11.);

    t &= u;

    assert!(t == v)
}
#[test]
fn reflect() {
    let i = Vec3::new(3., -1., 2.);
    let n = Vec3::new(1., 2., -1.);
    let r = Vec3::new(5., 3., 0.);

    assert!(i | n == r)
}
#[test]
fn reflecteq() {
    let mut i = Vec3::new(3., -1., 2.);
    let n = Vec3::new(1., 2., -1.);
    let r = Vec3::new(5., 3., 0.);

    i |= n;

    assert!(i == r)
}
#[test]
fn neg() {
    let u = Vec3::new(6., 8., 2.);
    let v = Vec3::new(-6., -8., -2.);

    assert!(-u == v)
}
#[test]
fn unit_len() {
    let t = Vec3::new(6., 8., 2.);

    assert!(t.unit().len() == 1.)
}
#[test]
fn normalize_len() {
    let mut t = Vec3::new(6., 8., 2.);

    t.normalize();

    assert!(t.len() == 1.)
}