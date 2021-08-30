use nalgebra::{Norm, Vec3, Dot};

use crate::Color;
use crate::material::Scattering;
use crate::ray::Ray;
use crate::hittable::HitRecord;

pub struct Metal {
    pub albedo: Color
}

impl Metal {
    fn reflect (in_dir: Vec3<f64>, normal_dir: Vec3<f64>) -> Vec3<f64>{
        return in_dir - (in_dir.dot(&normal_dir) * normal_dir) * 2.;
    }
}

impl Scattering for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord, attenuation: &mut Color, ray_out: &mut Ray) -> bool {
        let normal = (hit.compute_normal)();
        let reflected = Metal::reflect(ray_in.dir().normalize(), normal);
        *ray_out = Ray::new(hit.pos, reflected);
        *attenuation = self.albedo;
        ray_out.dir().dot(&normal) > 0.
    }
}