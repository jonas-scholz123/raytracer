pub mod lambertian;
pub mod metal;
pub mod dielectric;

use nalgebra::{Dot, Norm, Vec3};

use crate::ray::Ray;
use crate::hittable::{HitRecord};
use crate::Color;

pub trait Scattering {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord, attenuation: &mut Color, ray_out: &mut Ray) -> bool;
}

fn reflect (in_dir: &Vec3<f64>, normal_dir: &Vec3<f64>) -> Vec3<f64>{
    return *in_dir - (in_dir.dot(normal_dir) * *normal_dir) * 2.;
}

fn refract(in_dir: &Vec3<f64>, normal: &Vec3<f64>, refraction_ratio: f64) -> Vec3<f64> {

    let cos_theta = (-in_dir.dot(normal)).min(1.);
    let r_out_perp = refraction_ratio * (*in_dir + cos_theta * *normal);
    let r_out_parallel = -(1. - r_out_perp.sqnorm()).abs().sqrt() * *normal;
    r_out_parallel + r_out_perp
}

pub trait RandMaterial {
    fn random() -> Box<dyn Scattering + Send + Sync>;
}