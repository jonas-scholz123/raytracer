use nalgebra::{Dot, center};

use crate::Vec3;
use super::Intersectable;

use crate::ray::Ray;
use crate::Float;

struct Sphere {
    center: Vec3<Float>,
    radius: Float
}

impl Sphere {
    fn center(&self) -> &Vec3<Float> {&self.center}
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Float> {
        let l: Vec3<Float> = *self.center() - *ray.origin();
        let adj = l.dot(&ray.dir());
        let d2 = l.dot(&l) - (adj * adj);
        let radius2 = self.radius * self.radius;

        if d2 > radius2 {
            return None;
        }

        let thc = (radius2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;

        if t0 < 0.0 && t1 < 0.0 {
            return None;
        }

        let distance = if t0 < t1 { t0 } else { t1 };
        Some(distance)
    }
}