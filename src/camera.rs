use nalgebra::{Cross, Norm};

use crate::{ray::Ray};
use crate::Vec3;
use crate::utils::RandVec;

pub struct Camera {
    origin: Vec3<f64>,
    vertical: Vec3<f64>,
    horizontal: Vec3<f64>,
    lower_left_corner: Vec3<f64>,
    lens_radius: f64,
    u: Vec3<f64>,
    v: Vec3<f64>,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3<f64>,
        lookat: Vec3<f64>,
        vup: Vec3<f64>,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {

        // vfov: f64 vertical field of view in degrees
        let theta = vfov.to_radians();
        let height = (theta/2.).tan();
        let viewport_height = 2. * height;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;

        Camera {
            origin: lookfrom,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lookfrom - horizontal/2. - vertical/2. - focus_dist * w,
            lens_radius: aperture/2.,
            u: u,
            v: v,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::rand_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        let dir = self.lower_left_corner + self.horizontal * s + self.vertical * t  - self.origin;
        Ray::new(self.origin + offset, dir - offset)
    }
}

impl Default for Camera {
    fn default() -> Camera {
        let lookfrom = Vec3::new(0., 0., 0.);
        let lookat = Vec3::new(0., 0., -1.);
        let dist_to_focus = (lookfrom - lookat).norm();

        Camera::new(
            lookfrom,
            lookat,
            Vec3::new(0., 1., 0.),
            90.,
            16./9.,
            2.,
            dist_to_focus
        )
    }
}