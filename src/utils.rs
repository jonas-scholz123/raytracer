use nalgebra::Norm;
use rand::{Rng, prelude::ThreadRng};

use crate::{Vec3, VecN};

pub trait RandVec {
    fn rand(rng: &mut ThreadRng, min: f64, max: f64) -> VecN;
    fn rand_unit(rng: &mut ThreadRng) -> VecN;
}

pub trait NearZero {
    fn near_zero(self: &Self) -> bool;
}

impl NearZero for Vec3<f64> {
    fn near_zero(self: &Self) -> bool {
        self.norm() < 1e-8
    }
}

impl RandVec for Vec3<f64> {
    fn rand(rng: &mut ThreadRng, min: f64, max: f64) -> VecN {
        let x = rng.gen_range(min..max);
        let y = rng.gen_range(min..max);
        let z = rng.gen_range(min..max);

        Vec3::new(x, y, z)
    }

    fn rand_unit(rng: &mut ThreadRng) -> VecN {
    // generates a random vector with smaller than unit length
        loop {
            let candidate = Self::rand(rng, -1., 1.);
            if candidate.sqnorm() < 1. {
                return candidate;
            };
        }
    }
}