use crate::{objects::HitRecord, ray::Ray, vec3::{Color3, Vec3}};

use super::Material;

pub struct Lambertian {
    albedo: Color3,
}

impl Lambertian {
    pub fn new(albedo: Color3) -> Self {
        Self {
            albedo,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Color3, r_scattered: &mut Ray) -> bool {
        let mut scatter_dir: Vec3 = rec.normal + Vec3::rand_unit();

        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }

        *r_scattered = Ray::new(rec.p, scatter_dir);
        *attenuation = self.albedo;
        true
    }
}