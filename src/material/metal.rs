use nalgebra::{Norm, Vec3, Dot};

use crate::Color;
use crate::material::Scattering;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::utils::RandVec;
use super::reflect;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Scattering for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord, attenuation: &mut Color, ray_out: &mut Ray) -> bool {
        let normal = (hit.compute_normal)();
        let reflected = reflect(&ray_in.dir().normalize(), &normal);
        *ray_out = Ray::new(hit.pos, reflected + Vec3::rand_unit() * self.fuzz);
        *attenuation = self.albedo;
        ray_out.dir().dot(&normal) > 0.
    }
}