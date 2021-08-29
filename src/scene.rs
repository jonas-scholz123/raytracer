use std::cmp::min;

use crate::{hittable::{HitRecord, sphere::Sphere}, material::lambertian::Lambertian};
use crate::Ray;
use crate::Vec3;
use crate::hittable::{Hittable};
use rayon::prelude::*;


pub struct Scene
{
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub hittables: Vec<Box<dyn Hittable + Send + Sync>> // TODO: make this a vector of references with lifetimes
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

    pub fn add_hittable(&mut self, hittable: Box<dyn Hittable + Send + Sync>) {
        self.hittables.push(hittable);
    }

    pub fn next_hit(&self, ray: &Ray) -> Option<HitRecord> {
        // Use the Ordering of HitRecord to find the earliest hit and return it
        self.hittables
           .iter()
           .filter_map(|hittable| hittable.compute_hit(ray))
           .min()
    }
}

#[test]
fn can_render_scene() {
    let mut scene = Scene::new(800, 600, 90.);

    let sphere = Box::new(Sphere {
        center: Vec3::new(0.0, 0.0, -5.0),
        radius: 1.0,
        color: Vec3::new(0.4, 1.0, 0.4),
        material: Box::new(Lambertian{albedo: Vec3::new(0., 0., 0.)}),
    });

    scene.add_hittable(sphere);

    //let img = scene.render();
    //assert_eq!(scene.width, img.width());
    //assert_eq!(scene.height, img.height());
}