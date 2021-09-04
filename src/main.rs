use camera::Camera;
use hittable::Hittable;
use hittable::sphere::Sphere;
use image::{ImageBuffer, Rgb};
use material::{RandMaterial, Scattering};
use material::{dielectric::Dielectric, lambertian::Lambertian};
use material::metal::Metal;
use nalgebra::{Norm, Vec3};
use rand::{Rng, thread_rng};
use ray::Ray;
use scene::Scene;
use std::{time::Instant};
use rayon::prelude::*;
use utils::{RandVec, rand_f64, rand_f64_range};


extern crate image;
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

fn ray_color(ray: &Ray, scene: &Scene, depth: u32) -> Color {

    //println!("depth: {}", depth);
    if depth <= 0 {
        return Vec3::new(0., 0., 0.);
    }

    match scene.next_hit(&ray)  {
        Some(hit) => {
            
            let mut ray_out = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.));
            let mut attenuation = Color::new(0., 0., 0.);

            if hit.material.scatter(&ray, &hit, &mut attenuation, &mut ray_out) {

                return attenuation * ray_color(&ray_out, &scene, depth - 1)
            }
            return Color::new(0., 0., 0.);
        },
        None => {
            let unit_dir = ray.dir().normalize();
            let t = 0.5 * (unit_dir.y + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) 
            + Vec3::new(0.5.into(), 0.7.into(), 1.0.into()) * t
        }
    }
}

fn render_pixel(coords: (u32, u32), scene: &Scene, cam: &Camera, n_samples: u32) -> Rgb<u8>{
    let mut pixel_color = Vec3::new(0., 0., 0.);

    let mut rng = rand::thread_rng();

    let i = coords.0 as f64;
    let j = coords.1 as f64;

    for _ in 0..n_samples{
        rng.gen::<f64>();

        let u = (i + rng.gen::<f64>()) / (scene.width - 1) as f64;
        let v = (j + rng.gen::<f64>()) / (scene.height - 1) as f64;

        let r = cam.get_ray(u, v);
        pixel_color += ray_color(&r, &scene, 50);
    }

    pixel_color /= n_samples as f64;

    // we take sqrt as gamma correction (as per the tutorial)
    let red = (pixel_color.x.sqrt() * 256.) as u8;
    let green = (pixel_color.y.sqrt() * 256.) as u8;
    let blue = (pixel_color.z.sqrt() * 256.) as u8;

    image::Rgb([red, green, blue])

}

fn render(scene: &Scene, cam: &Camera, n_samples: u32) {
    // prepare a Vec of all pixel coordinates for parallelisation
    let mut coords: Vec<(u32, u32)> = Vec::new();
    for i in 0..scene.width {
        for j in (0..scene.height).rev() {
            coords.push((i, j));
        }
    }

    // Actually render the image
    let colors: Vec<Rgb<u8>> = coords
        .par_iter()
        .map(|pos| render_pixel(*pos, &scene, &cam, n_samples))
        .collect();

    // Transfer the rendered colours to the ImageBuffer
    let mut imgbuf: ImageBuffer<Rgb<u8>, _> = ImageBuffer::new(scene.width, scene.height);
    for (i, j, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = colors[(i * scene.height) as usize + j as usize];
    }

    imgbuf.save("./images/background.png").unwrap();
}

fn random_material() -> Box<dyn Scattering + Send + Sync> {
    return match rand_f64() {
        nr if nr < 0.8 => Lambertian::random(),
        nr if nr < 0.95 => Metal::random(),
        _ => Dielectric::random(),
    };
}

fn render_random_image() {
    let aspect_ratio = 16.0/9.0;
    let width = 800;
    let height = (width as f64 / aspect_ratio) as u32;
    let fov = 2.;
    let samples_per_pixel = 500;

    let lookfrom = Vec3::new(13., 2., 3.);
    let lookat = Vec3::new(0., 0., 0.);

    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0., 1., 0.),
        20.,
        aspect_ratio,
        0.1,
        10.
        //(lookfrom - lookat).norm(),
    );

    let mut scene = Scene::new(width, height, fov);
    let my_ground = Box::new(Lambertian {albedo: Vec3::new(0.5, 0.5, 0.5)});
    let ground = Sphere{
        center: Vec3::new(0., -1000., 0.),
        radius: 1000.,
        material: my_ground,
    };

    for a in -11 .. 11 {
        for b in -11 .. 11 {

            let a = a as f64;
            let b = b as f64;
            let center = Vec3::new(a + 0.9 * rand_f64(), 0.2, b + 0.9 * rand_f64());

            if (center - Vec3::new(4., 0.2, 0.)).norm() <= 0.9 {
                continue;
            }

            scene.add_hittable(
                Box::new(Sphere{
                    center: center,
                    radius: 0.2,
                    material: random_material(), 
                })
            )

        }
    }

    scene.add_hittable(Box::new(ground));

    scene.add_hittable(Box::new(
        Sphere {
            center: Vec3::new(0., 1., 0.),
            radius: 1.,
            material: Dielectric::random()
        }));
    
    scene.add_hittable(Box::new(
        Sphere {
            center: Vec3::new(4., 1., 0.),
            radius: 1.,
            material: Box::new(Metal {
                albedo: Vec3::new(0.7, 0.6, 0.5),
                fuzz: 0.
            })
        }));
    
    scene.add_hittable(Box::new(
        Sphere {
            center: Vec3::new(-4., 1., 0.),
            radius: 1.,
            material: Lambertian::random()
        }));

    render(&scene, &cam, samples_per_pixel)

}