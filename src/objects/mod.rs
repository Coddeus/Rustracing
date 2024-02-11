use crate::Interval;

use super::ray::*;
use super::vec3::*;

mod sphere;
pub use sphere::*;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            t: 0.,
            front: false,
        }
    }

    pub fn set_normal(&mut self, r: &Ray, n_outward: Vec3) {
        self.front = r.dir() * n_outward < 0.;
        self.normal = if self.front { n_outward } else { -n_outward }
    }
}

pub trait Object {
    /// Checks whether `self` is on the path of `r`, for a t value between `tmin` and `tmax`.
    /// If so, updates `rec` accordingly.
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool;
    // Each object alaso needs an initializer, fn new()
}


pub struct Objects<'a>(Vec<&'a (dyn Object + 'a)>);

impl<'obj> Objects<'obj> {
    pub fn new() -> Self {
        Objects(vec![])
    }

    pub fn add(&mut self, object: &'obj impl Object) {
        self.0.push(object)
    }

    pub fn clear(&mut self) {
        self.0 = vec![]
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new();
        let mut anyhit: bool = false;

        for obj in self.0.iter() {
            if obj.hit(r, &ray_t, &mut temp_rec) {
                anyhit = true;
                ray_t.max = temp_rec.t;
            }
        }
        
        if anyhit { *rec = temp_rec; }
        anyhit
    }
}