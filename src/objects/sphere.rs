use std::rc::Rc;

use crate::materials::Material;
use crate::utils::Interval;
use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};

use super::{HitRecord, Object};


pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Material>
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat,
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
        rec.mat = self.mat.clone();

        true
    }
}