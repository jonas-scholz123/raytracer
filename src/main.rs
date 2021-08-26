use camera::Camera;
use hittable::sphere::{self, Sphere};
use nalgebra::{Norm, Vec3};
use rand::{Rng, prelude::ThreadRng};
use ray::Ray;
use scene::Scene;
use std::time::Instant;
use utils::RandVec;

extern crate image;
mod ray;
mod hittable;
mod scene;
mod io;
mod camera;
mod utils;
type VecN = Vec3<f64>;
type Color = Vec3<f64>;

fn main() {
    let before = Instant::now();
    make_background();
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn ray_color(ray: &Ray, scene: &Scene, rng: &mut ThreadRng, depth: u32) -> Color {

    //println!("{}", depth);
    if depth <= 0 {
        return Vec3::new(0., 0., 0.);
    }

    match scene.next_hit(&ray)  {
        Some(hit) => {
            //let target = hit.pos + (hit.compute_normal)() + Vec3::<f64>::rand_unit(rng);
            let new_direction = (hit.compute_normal)() + Vec3::<f64>::rand_unit(rng).normalize();
            let next_ray = Ray::new(hit.pos, new_direction);
            return 0.5 * ray_color(&next_ray, &scene, rng, depth - 1);
            //return ((hit.compute_normal)() + Color::new(1., 1., 1.)) * 0.5;
        },
        None => {
            let unit_dir = ray.dir().normalize();
            let t = 0.5 * (unit_dir.y + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) 
            + Vec3::new(0.5.into(), 0.7.into(), 1.0.into()) * t
        }
    }
}

fn render(scene: &Scene, cam: &Camera, n_samples: u32) {
    let mut imgbuf = image::ImageBuffer::new(scene.width as u32, scene.height as u32);
    let mut rng = rand::thread_rng();
    let mut count = 0;

    for (i, j, pixel) in imgbuf.enumerate_pixels_mut() {
        count += 1;

        let i = i as f64;
        let j = scene.height as f64 - j as f64;

        let mut pixel_color = Vec3::new(0., 0., 0.);

        for _ in 0..n_samples{
            rng.gen::<f64>();

            let u = (i + rng.gen::<f64>()) / (scene.width - 1) as f64;
            let v = (j + rng.gen::<f64>()) / (scene.height - 1) as f64;

            let r = cam.get_ray(u, v);
            pixel_color += ray_color(&r, &scene, &mut rng, 50);
            //println!("colour: {}", pixel_color)
        }

        if count % 10000 == 0 {
            println!("{}%", 100 * count / (scene.width * scene.height));
        }

        pixel_color /= n_samples as f64;

        // pixel is a reference, we set the dereferenced pixel to image::Rgb([r, 0, b])

        // we take sqrt as gamma correction (as per the tutorial)
        let red = (pixel_color.x.sqrt() * 256.) as u8;
        let green = (pixel_color.y.sqrt() * 256.) as u8;
        let blue = (pixel_color.z.sqrt() * 256.) as u8;

        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save("./images/background.png").unwrap();
}

fn make_background() {
    let aspect_ratio = 16.0/9.0;
    let width = 800;
    let height = (width as f64 / aspect_ratio) as u32;
    let fov = 3.;
    let samples_per_pixel = 20;

    let cam = Camera::default();

    let mut scene = Scene::new(width, height, fov);

    let sphere1 = Sphere{ center: Vec3::new(0., 0., -1.), radius: 0.5, color: Vec3::new(0., 0., 0.)};
    let sphere2 = Sphere{ center: Vec3::new(0., - 100.5, -1.), radius: 100., color: Vec3::new(0., 0., 0.)};
    //let sphere3 = Sphere{ center: Vec3::new(0.5, 0.5, -1.), radius: 0.1, color: Vec3::new(0., 0., 0.)};

    scene.add_hittable(Box::new(sphere1));
    scene.add_hittable(Box::new(sphere2));
    //scene.add_hittable(Box::new(sphere3));

    render(&scene, &cam, samples_per_pixel)

}