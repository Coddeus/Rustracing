use super::vec3::*;

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    } 

    pub fn orig(&self) -> Point3 {
        self.orig
    }
    pub fn dir(&self) -> Vec3 {
        self.dir
    }
}