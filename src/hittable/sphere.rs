use nalgebra::{Dot, Norm};

use crate::Vec3;
use crate::Vec;
use super::{Hittable, HitRecord};

use crate::ray::Ray;
use crate::Float;

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vec3<Float>,
    pub radius: Float,
    pub color: Vec3<Float>
}

impl Sphere {
    fn center(&self) -> &Vec3<Float> {&self.center}
}

impl Hittable for Sphere {
    fn hit_time(&self, ray: &Ray) -> Option<HitRecord> {
        // TODO: return time/None, later keep track of shortest collision time for ray
        // and also keep track of shortest collision Hittable. Then only calc
        // normal for shortest

        // CHECK: do I need t min t max and other root?
        let oc = *ray.origin() - self.center;
        let dir2 = ray.dir().sqnorm();
        let proj = oc.dot(ray.dir());
        let c = oc.sqnorm() - self.radius * self.radius;
        let discriminant = proj * proj - dir2 * c;
        if discriminant < 0. {
            return None;
        } else {
            let t = -(proj + discriminant.sqrt()) / dir2;
            return Some(HitRecord{
                pos: ray.at(t),
                normal: None,
                time: t,
                ray: *ray,
            })
        }
    }

    fn compute_normal(&self, mut hit: &mut HitRecord) {
        let outward_normal = (hit.pos - self.center) / self.radius;
        let is_internal = hit.ray.dir().dot(&outward_normal) < 0.;
        hit.normal = if is_internal { Some(outward_normal) } else { Some(-outward_normal) };
    }
}