use crate::image::DynamicImage;
use crate::hittable::{sphere::Sphere, Hittable};
use crate::{Vec3, Float};
use crate::image::GenericImageView;

pub struct Scene<T>
where T: Hittable + Copy
{
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub hittables: Vec<T> // TODO: make this a vector of references with lifetimes
}

//TODO: Fix, polymorphism
impl<T> Scene<T> 
where T: Hittable + Copy
{   
    pub fn new(width: u32, height: u32, fov: Float) -> Scene<T> {
        Scene {
            width: width,
            height: height,
            fov: fov,
            hittables: Vec::new(),
        }
    }

    pub fn add_hittable(&mut self, hittable: &T) {
        self.hittables.push(*hittable);
    }

    pub fn render(&self) -> DynamicImage {
        DynamicImage::new_rgb8(self.width, self.height)
    }
}

#[test]
fn can_render_scene() {
    let mut scene = Scene::new(800, 600, 90.);

    let sphere = Sphere {
        center: Vec3::new(0.0, 0.0, -5.0),
        radius: 1.0,
        color: Vec3::new(0.4, 1.0, 0.4)
    };

    scene.add_hittable(&sphere);

    let img = scene.render();
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
}