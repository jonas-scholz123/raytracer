use nalgebra::Dot;
use nalgebra::Norm;
use rand::Rng;
use rand::thread_rng;

use crate::material::Scattering;
use crate::Color;
use crate::Ray;
use crate::hittable::HitRecord;

use super::reflect;
use super::refract;

pub struct Dielectric {
    pub reflection_index: f64,
}

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64{
        //Use Schlick's approximation for reflectance.
        let r0 = (1. - ref_idx) / (1. + ref_idx);
        let r0 = r0 * r0;
        r0 + (1. - r0) * (1. - cosine).powi(5)
    }

    fn will_reflect(cosine: f64, ref_idx: f64) -> bool {
        let mut rng = thread_rng();
        Dielectric::reflectance(cosine, ref_idx) > rng.gen_range(0 as f64 ..1 as f64)
    }
}

impl Scattering for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord, attenuation: &mut Color, ray_out: &mut Ray) -> bool {
        *attenuation = Color::new(1., 1., 1.);
        let refraction_ratio = if (hit.compute_is_external)()
            {1./self.reflection_index}
            else {self.reflection_index};

        let normal = (hit.compute_normal)();
        let unit_dir = ray_in.dir().normalize();
        let cos_theta = (- unit_dir.dot(&normal)).min(1.);

        let sin_theta = (1. - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract || Dielectric::will_reflect(cos_theta, refraction_ratio)
            {reflect(&unit_dir, &normal)}
            else {refract(&unit_dir, &normal, refraction_ratio)};
        

        *ray_out = Ray::new(hit.pos, direction);
        true
    }
}