use std::ops::{Add, Mul};

use nalgebra::{BaseFloat, Norm};
use nalgebra::Vec3 as Vec3;

extern crate image;
mod ray;
type Float = f32;

fn main() {
    make_background();
}

fn ray_color<T>(ray: ray::Ray<T>) -> Vec3<T> 
    where T: Mul + Add + BaseFloat,
          f64: Into<T>
{
    let unit_dir = ray.dir().normalize();
    let t = 0.5.into() * (unit_dir.y + 1.0.into());
    Vec3::new(1.0.into(), 1.0.into(), 1.0.into()) * (1.0.into() - t) 
    + Vec3::new(0.5.into(), 0.7.into(), 1.0.into()) * t
}

//fn make_image() {
//    let imgx = 800;
//    let imgy = 800;
//
//    // make image buffer
//    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
//    let mut count = 0;
//
//    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
//        count += 1;
//        let r = (256 * x/imgx) as u8;
//        let g = (256 * y/imgy) as u8;
//        let b = 80 as u8; 
//
//        // pixel is a reference, we set the dereferenced pixel to image::Rgb([r, 0, b])
//        *pixel = image::Rgb([r, g, b]);
//        
//        if count % 1000 == 0 {
//            println!("{}%", 100 * count / (imgx * imgy));
//        }
//    }
//    
//    println!("Finished");
//    imgbuf.save("./images/background.png").unwrap()
//}

fn make_background() {
    let aspect_ratio = 16.0/9.0;
    let width = 800 as f32;
    let height = (width / aspect_ratio) as i32;

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
        let i = i as f64;
        let j = j as f64;
        let u = i / (width as f64 - 1.);
        let v = j / (height as f64 - 1.);

        let r = ray::Ray::new(origin, lower_left_corner + u as f64 * horizontal + v as f64 * vertical - origin);
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