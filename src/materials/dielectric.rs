use fastrand::f64;

use crate::{objects::HitRecord, ray::Ray, vec3::{Color3, Vec3}};

use super::Material;

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self {
            ir,
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0: f64 = 1.0 - ref_idx / (1.0 + ref_idx);
        let r0 = r0*r0;
        r0 + (1.0-r0) * (1.0-cosine).powi(5)

    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color3, r_scattered: &mut Ray) -> bool {
        *attenuation = Color3::new(1.0, 1.0, 1.0);
        let refraction_ratio: f64 = if rec.front { 1.0/self.ir } else { self.ir };

        let unit_dir: Vec3 = r_in.dir().unit();

        let cos_theta: f64 = ((-unit_dir) * rec.normal).min(1.0);
        let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;
        let dir: Vec3;

        if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > f64() {
            dir = unit_dir | rec.normal;
        } else {
            dir = unit_dir.refract(rec.normal, refraction_ratio);
        }

        *r_scattered = Ray::new(rec.p, dir);
        true
    }
}