use std::iter::Scan;

use nalgebra::Norm;
use nalgebra::Vec3;
use rand::prelude::ThreadRng;

use crate::material::Scattering;
use crate::Color;
use crate::Ray;
use crate::hittable::HitRecord;
use crate::utils::RandVec;

pub struct Lambertian {
    pub albedo: Color,
}

impl Scattering for Lambertian {
    fn scatter(&self, rng: &mut ThreadRng, ray_in: &Ray, hit: &HitRecord, attenuation: &mut Color, ray_out: &Ray) -> bool {
       let scatter_dir = (hit.compute_normal) () + Vec3::<f64>::rand_unit(rng).normalize();
       let scattered = Ray::new(hit.pos, scatter_dir);
       let mut rng = rand::thread_rng();
       *attenuation = self.albedo.clone();
       return true;
    }
}