use nalgebra::Norm;
use nalgebra::Vec3;

use crate::material::Scattering;
use crate::Color;
use crate::Ray;
use crate::hittable::HitRecord;
use crate::utils::NearZero;
use crate::utils::RandVec;

pub struct Lambertian {
    pub albedo: Color,
}

impl Scattering for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit: &HitRecord, attenuation: &mut Color, ray_out: &mut Ray) -> bool {
        
        let normal = (hit.compute_normal) ();
        let mut scatter_dir = normal + Vec3::<f64>::rand_unit().normalize();

        // catch NaNs before they happen
        if scatter_dir.near_zero() {
            scatter_dir = normal;
        }

        *ray_out = Ray::new(hit.pos, scatter_dir);
        *attenuation = self.albedo.clone();
        return true;
    }
}