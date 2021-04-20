use nalgebra::{Cross, Norm};

use crate::Float;
use crate::Vec3;
use std::f64::consts::PI;

pub struct Camera {
    lower_left_corner: Vec3<Float>,
    horizontal: Vec3<Float>,
    vertical: Vec3<Float>,
    origin: Vec3<Float>,
    lens_radius: f64,

    u: Vec3<Float>,
    v: Vec3<Float>,
    w: Vec3<Float>,
}

impl Camera {
    pub fn new(lookfrom: Vec3<Float>, lookat: Vec3<Float>, vup: Vec3<Float>, vfov: Float,
            aspect: Float, aperture: Float, focus_dist: Float) -> Camera {

        let lens_radius = aperture/2.0;
        let theta = vfov * PI; 
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;

        let origin = lookfrom;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let lower_left_corner = origin - focus_dist * (half_width * u + half_height * v + w);
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;

        Camera {
            lower_left_corner : lower_left_corner,
            horizontal : horizontal,
            vertical : vertical,
            origin : origin,
            lens_radius : lens_radius,
            u : u,
            v : v,
            w : w,
        }
    }
}