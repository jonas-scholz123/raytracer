pub mod lambertian;
pub mod metal;

use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::Color;

pub trait Scattering {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord, attenuation: &mut Color, ray_out: &mut Ray) -> bool;
}