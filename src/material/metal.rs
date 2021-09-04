use nalgebra::{Norm, Vec3, Dot};

use crate::{Color, utils::rand_f64_range};
use crate::material::Scattering;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::utils::RandVec;
use super::{RandMaterial, reflect};

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

impl RandMaterial for Metal {
    fn random() -> Box<dyn Scattering + Send + Sync> {
        let albedo = Vec3::rand(0.5, 1.);
        let fuzz = rand_f64_range(0., 0.5);
        Box::new(Metal {albedo: albedo, fuzz: fuzz})
    }
}