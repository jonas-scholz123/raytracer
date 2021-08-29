use nalgebra::{Dot, Norm};

use crate::VecN;
use crate::material::Scattering;
use super::{Hittable, HitRecord};

use crate::ray::Ray;
pub struct Sphere {
    pub center: VecN,
    pub radius: f64,
    pub color: VecN,
    pub material: Box<dyn Scattering + Send + Sync>,
}

impl Sphere {
    fn center(&self) -> &VecN {&self.center}
}

impl Hittable for Sphere {
    fn compute_hit(&self, ray: &Ray) -> Option<HitRecord> {
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
        } 

        // TODO: Move this into config

        // The 0.0001 prevents rays that actually hit the object at t = 0 but are calculated as
        // -0.0000000001 from falsely impacting the final image. We filter the floating point
        // approximation error.
        let t_min = 0.0001;
        let t_max = f64::INFINITY;
        
        let root = (-proj - discriminant.sqrt()) / dir2;
        if root < t_min || t_max < root {
            let root = (-proj + discriminant.sqrt()) / dir2;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = -(proj + discriminant.sqrt()) / dir2;

        // clone all the params required for compute_normal
        // and 'move' them into closure
        let pos = ray.at(t);
        let center = self.center;
        let radius = self.radius;
        let dir = *ray.dir();

        return Some(HitRecord{
            pos: ray.at(t),
            time: t,
            ray: *ray,
            compute_normal: Box::new(move || {
                let outward_normal = (pos - center) / radius;
                let is_internal = dir.dot(&outward_normal) < 0.;
                if is_internal {outward_normal} else {-outward_normal}
            }),
            material: &self.material
        })
    }
}