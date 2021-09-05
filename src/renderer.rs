use crate::Color;
use crate::camera::Camera;
use image::{ImageBuffer, Rgb};
use nalgebra::{Norm, Vec3};
use rand::{Rng};
use crate::ray::Ray;
use crate::scene::Scene;
use rayon::prelude::*;

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

pub fn render(scene: &Scene, cam: &Camera, n_samples: u32) {
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

    imgbuf.save("./images/random_scene.png").unwrap();
}