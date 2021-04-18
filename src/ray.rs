use std::ops::{Add, Mul};

use nalgebra::{BaseFloat, Vec3 as Vec3};
use crate::Float;

#[derive(Clone, Copy)]
pub struct Ray<T> 
    where T: Mul + Add + BaseFloat
{
    origin: Vec3<T>,
    dir: Vec3<T>
}

impl<T> Ray<T> 
    where T: Mul + Add + BaseFloat
{
    pub fn new(origin: Vec3<T>, dir: Vec3<T>) -> Ray<T>{
        Ray {origin, dir}
    }

    pub fn origin(&self) -> &Vec3<T> {
        &self.origin
    }

    pub fn dir(&self) -> &Vec3<T> {
        &self.dir
    }

    pub fn at(&self, t: T) -> Vec3<T> 
        where T: Mul + Add, 
              Vec3<T>: Mul<T, Output = Vec3<T>> + Add<Output = Vec3<T>>
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