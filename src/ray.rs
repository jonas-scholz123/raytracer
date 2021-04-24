use nalgebra::{Vec3};
use crate::f64;

#[derive(Clone, Copy)]
pub struct Ray 
{
    origin: Vec3<f64>,
    dir: Vec3<f64>
}

impl Ray { 
    pub fn new(origin: Vec3<f64>, dir: Vec3<f64>) -> Ray{
        Ray {origin, dir}
    }

    pub fn origin(&self) -> &Vec3<f64> {
        &self.origin
    }

    pub fn dir(&self) -> &Vec3<f64> {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Vec3<f64>
    {
        self.origin + self.dir * t
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at() {
        let origin = Vec3::new(12.4, 0., 0.);
        let dir = Vec3::new(3., 0., 0.);
        let ray = Ray::new(origin, dir);
        assert_eq!(ray.at(2.), Vec3::new(18.4, 0., 0.));
    }
}