use std::cmp::min;

use crate::{hittable::{HitRecord, sphere::Sphere}};
use crate::Ray;
use crate::Vec3;
use crate::hittable::{Hittable};

pub struct Scene
{
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub hittables: Vec<Box<dyn Hittable>> // TODO: make this a vector of references with lifetimes
}

//TODO: Fix, polymorphism
impl Scene {   
    pub fn new(width: u32, height: u32, fov: f64) -> Scene {
        Scene {
            width: width,
            height: height,
            fov: fov,
            hittables: Vec::new(),
        }
    }

    pub fn add_hittable(&mut self, hittable: Box<dyn Hittable>) {
        self.hittables.push(hittable);
    }

    pub fn next_hit(&self, ray: &Ray) -> Option<HitRecord> {
        let mut next_hit = HitRecord::default();

        for hittable in &self.hittables {
            match hittable.compute_hit(ray) {
                Some(record) => next_hit = min(next_hit, record),
                None => continue
            }
        }

        if next_hit.time == f64::INFINITY {
            return None;
        }
        // Only compute the normal of the next hit.
        next_hit.normal = Some((next_hit.compute_normal)());
        //next_hit.hittable.compute_normal(&mut next_hit);
        Some(next_hit)
    }
    
    pub fn render() {}
}

#[test]
fn can_render_scene() {
    let mut scene = Scene::new(800, 600, 90.);

    let sphere = Box::new(Sphere {
        center: Vec3::new(0.0, 0.0, -5.0),
        radius: 1.0,
        color: Vec3::new(0.4, 1.0, 0.4)
    });

    scene.add_hittable(sphere);

    //let img = scene.render();
    //assert_eq!(scene.width, img.width());
    //assert_eq!(scene.height, img.height());
}