use crate::{objects::HitRecord, ray::Ray, vec3::Color3};

mod lambertian;
pub use lambertian::*;
mod metal;
pub use metal::*;

pub trait Material {
    /// Returns whether a ray is scattered. If it is, modifies `attenuation` and `r_scattered` for the material.
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color3, r_scattered: &mut Ray) -> bool;
}