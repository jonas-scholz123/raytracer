use crate::{VecN, Vec3, sphere::Sphere};
use crate::ray::Ray;
use std::cmp::Ordering;

pub mod sphere;

pub struct HitRecord {
    pub pos: VecN,
    pub normal: Option<VecN>,
    pub time: f64,
    pub ray: Ray,
    pub hittable: Box<dyn Hittable>,
    pub compute_normal: Box<dyn Fn() -> VecN>
    //TODO: replace hittable by a function: fn calc_normal (HitRecord) -> normal
    //      which depends on the hit object, when calculating normal, only have to
    //      call hit.normal = Some(hit.calc_normal)
}

impl Default for HitRecord {
    fn default() -> HitRecord {
        let null: Vec3<f64> = Vec3::new(0., 0., 0.);
        HitRecord {
            pos: null,
            normal: None,
            time: f64::INFINITY,
            ray: Ray::new(null, null),
            hittable: Box::new(Sphere{ center: null, radius: 0., color: null}),
            compute_normal: Box::new(move || {null})
        }
    }
}

impl PartialEq for HitRecord {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl PartialOrd for HitRecord {
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

impl Eq for HitRecord {}

impl Ord for HitRecord {
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

pub trait Hittable{
    fn compute_hit(&self, ray: &Ray) -> Option<HitRecord>;
}