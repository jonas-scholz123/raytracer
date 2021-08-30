use nalgebra::Norm;
use rand::{Rng, thread_rng};

use crate::{Vec3, VecN};

pub trait RandVec {
    fn rand(min: f64, max: f64) -> VecN;
    fn rand_unit() -> VecN;
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
    fn rand(min: f64, max: f64) -> VecN {
        let mut rng = thread_rng();
        let x = rng.gen_range(min..max);
        let y = rng.gen_range(min..max);
        let z = rng.gen_range(min..max);

        Vec3::new(x, y, z)
    }

    fn rand_unit() -> VecN {
    // generates a random vector with smaller than unit length
        loop {
            let candidate = Self::rand(-1., 1.);
            if candidate.sqnorm() < 1. {
                return candidate;
            };
        }
    }
}