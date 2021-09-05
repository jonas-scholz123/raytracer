use nalgebra::Vec3;
use ray::Ray;
use std::{time::Instant};
use random_scene::render_random_image;


extern crate image;
mod renderer;
mod random_scene;
mod ray;
mod hittable;
mod material;
mod scene;
mod io;
mod camera;
mod utils;
type VecN = Vec3<f64>;
type Color = Vec3<f64>;

fn main() {
    let before = Instant::now();
    render_random_image();
    println!("Elapsed time: {:.2?}", before.elapsed());
}
