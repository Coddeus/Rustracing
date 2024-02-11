use super::{Object, HitRecord};
use crate::{Interval, Point3, Ray, Vec3};


pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self {
            center,
            radius,
        }
    }
}

impl Object for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.orig() - self.center;
        let a: f64 = r.dir().len_squared();
        let half_b: f64 = oc * r.dir();
        let c: f64 = oc.len_squared() - self.radius * self.radius;
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0. {
            return false
        }
        let sqrtd: f64 = discriminant.sqrt();

        let mut root = (- half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (- half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let n_outward: Vec3 = (rec.p - self.center) / self.radius;
        rec.set_normal(r, n_outward);

        true
    }
}