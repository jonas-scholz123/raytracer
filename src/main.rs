use hittable::sphere;
use nalgebra::{Dot, Norm};
use nalgebra::Vec3 as Vec3;
use ray::Ray;

extern crate image;
mod ray;
mod hittable;
mod scene;
mod io;
mod camera;
type Float = f64;
type Vec = Vec3<Float>;
type Color = Vec3<Float>;

fn main() {
    make_background();
}

fn ray_color(ray: ray::Ray) -> Color {
    let t = sphere_collision_time(&Vec::new(0., 0., -1.), 0.5, ray);

    if t > 0. {
        let n = ray.at(t) - Vec::new(0., 0., -1.);
        return 0.5 * Color::new(n.x + 1., n.y + 1., n.z + 1.);
    }
    let unit_dir = ray.dir().normalize();
    let t = 0.5 * (unit_dir.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) 
    + Vec3::new(0.5.into(), 0.7.into(), 1.0.into()) * t
}

fn make_background() {
    let aspect_ratio = 16.0/9.0;
    let width = 800;
    let height = (width as Float / aspect_ratio) as i32;

    let viewport_height = 2.0;
    let viewport_width = (aspect_ratio * viewport_height) as f64;
    let focal_length = 1.0;

    let origin = Vec3::new(0., 0., 0.);
    let horizontal = Vec3::new(viewport_width, 0. as f64, 0. as f64);
    let vertical = Vec3::new(0. as f64, viewport_height as f64, 0. as f64);
    let lower_left_corner = origin - horizontal/2. - vertical/2. - Vec3::new(0., 0., focal_length);

    let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);

    let mut count = 0;
    for (i, j, pixel) in imgbuf.enumerate_pixels_mut() {
        count += 1;
        let i = i as Float;
        let j = j as Float;
        let u = i / (width - 1) as Float;
        let v = j / (height - 1) as Float;

        let r = ray::Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
        let pixel_color = ray_color(r);

        // pixel is a reference, we set the dereferenced pixel to image::Rgb([r, 0, b])

        let red = (pixel_color.x * 256.) as u8;
        let green = (pixel_color.y * 256.) as u8;
        let blue = (pixel_color.z * 256.) as u8;

        *pixel = image::Rgb([red, green, blue]);

        if count % 1000 == 0 {
            println!("{}%", 100 * count / (width as i32 * height));
        }
    }

    println!("Finished");
    imgbuf.save("./images/background.png").unwrap();
}

fn sphere_collision_time(center: &Vec, radius: Float, ray: Ray) -> Float {
    let oc = *ray.origin() - *center;
    let dir2 = ray.dir().sqnorm();
    let proj = oc.dot(ray.dir());
    let c = oc.sqnorm() - radius * radius;
    let discriminant = proj * proj - dir2 * c;
    if discriminant < 0. {
        return -1.;
    } else {
        return (-proj - discriminant.sqrt()) / dir2;
    }
}