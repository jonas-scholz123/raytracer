use crate::{Float, Vec};
use crate::ray::Ray;

pub mod sphere;

pub struct HitRecord {
    pub pos: Vec,
    pub normal: Option<Vec>,
    pub time: Float,
    pub ray: Ray,
}

pub trait Hittable {
    fn hit_time(&self, ray: &Ray) -> Option<HitRecord>;
    fn compute_normal(&self, hit: &mut HitRecord);
}