pub mod lambertian;

use nalgebra::Col;
use rand::prelude::ThreadRng;

use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::Color;

pub trait Scattering {
    fn scatter(&self, rng: &mut ThreadRng, ray_in: &Ray, hit: &HitRecord, attenuation: &mut Color, ray_out: &Ray) -> bool;
}