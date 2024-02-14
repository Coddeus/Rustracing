use crate::{objects::HitRecord, ray::Ray, vec3::{Color3, Vec3}};

use super::Material;

pub struct Metal {
    albedo: Color3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color3, r_scattered: &mut Ray) -> bool {
        let reflection_dir: Vec3 = r_in.dir().unit() | rec.normal;

        *r_scattered = Ray::new(rec.p, reflection_dir + self.fuzz * Vec3::rand_unit());
        *attenuation = self.albedo;
        r_scattered.dir() * rec.normal > 0.
    }
}