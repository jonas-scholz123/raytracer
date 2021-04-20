use nalgebra::{Vec3};
use crate::Float;

#[derive(Clone, Copy)]
pub struct Ray 
{
    origin: Vec3<Float>,
    dir: Vec3<Float>
}

impl Ray { 
    pub fn new(origin: Vec3<Float>, dir: Vec3<Float>) -> Ray{
        Ray {origin, dir}
    }

    pub fn origin(&self) -> &Vec3<Float> {
        &self.origin
    }

    pub fn dir(&self) -> &Vec3<Float> {
        &self.dir
    }

    pub fn at(&self, t: Float) -> Vec3<Float>
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