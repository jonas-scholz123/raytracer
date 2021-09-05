use crate::camera::Camera;
use crate::hittable::sphere::Sphere;
use crate::renderer::render;
use crate::material::{RandMaterial, Scattering};
use crate::material::{dielectric::Dielectric, lambertian::Lambertian};
use crate::material::metal::Metal;
use nalgebra::{Norm, Vec3};
use crate::scene::Scene;
use crate::utils::{rand_f64};

fn random_material() -> Box<dyn Scattering + Send + Sync> {
    return match rand_f64() {
        nr if nr < 0.5 => Lambertian::random(),
        nr if nr < 0.90 => Metal::random(),
        _ => Dielectric::random(),
    };
}

pub fn render_random_image() {
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