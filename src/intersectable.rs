use crate::Float;
use crate::ray::Ray;
use crate::Vec3;

pub mod sphere;

trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Float>;
}