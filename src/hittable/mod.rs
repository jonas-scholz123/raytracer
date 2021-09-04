use crate::{VecN};
use crate::ray::Ray;
use crate::material::Scattering;
use std::cmp::Ordering;

pub mod sphere;

pub struct HitRecord<'a> {
    pub pos: VecN,
    pub time: f64,
    pub ray: Ray,
    pub compute_normal: Box<dyn Fn() -> VecN>,
    pub material: &'a Box<dyn Scattering + 'a + Send + Sync>,
    pub compute_is_external: Box<dyn Fn() -> bool>,
}

impl<'a> PartialEq for HitRecord<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl<'a> PartialOrd for HitRecord<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.time == other.time {
            return Some(Ordering::Equal)
        }
        else if self.time > other.time {
            return Some(Ordering::Greater)
        }
        else {
            return Some(Ordering::Less)
        }
    }
}

impl<'a> Eq for HitRecord<'a> {}

impl<'a> Ord for HitRecord<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.time == other.time {
            return Ordering::Equal
        }
        else if self.time > other.time {
            return Ordering::Greater
        }
        else {
            return Ordering::Less
        }
    }
}

pub trait Hittable {
    fn compute_hit(&self, ray: &Ray) -> Option<HitRecord>;
}