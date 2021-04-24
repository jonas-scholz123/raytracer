use crate::{ray::Ray};
use crate::Vec3;

pub struct Camera {
    aspect_ratio: f64,
    viewport_height: f64,
    viewport_width: f64,
    focal_length: f64,
    origin: Vec3<f64>,
    vertical: Vec3<f64>,
    horizontal: Vec3<f64>,
    lower_left_corner: Vec3<f64>,
}

impl Default for Camera {
    fn default() -> Camera {
        let aspect_ratio = 16.0/9.0;
        let viewport_height = 2.0;
        let viewport_width = (aspect_ratio * viewport_height) as f64;

        let focal_length = 1.0;
        let origin = Vec3::new(0., 0., 0.);
        let horizontal = Vec3::new(viewport_width, 0. as f64, 0. as f64);
        let vertical = Vec3::new(0. as f64, viewport_height as f64, 0. as f64);
        let lower_left_corner = origin - horizontal/2. - vertical/2. - Vec3::new(0., 0., focal_length);

        Camera {
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            vertical,
            horizontal,
            lower_left_corner
        }
    }
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal * u
                                  +self.vertical * v  - self.origin)
    }
}