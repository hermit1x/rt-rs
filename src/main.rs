mod ray;

mod write_img;
mod common;
mod sphere;
mod hittable;
mod hittable_list;
mod interval;

use ray::Ray;
use write_img::write_jpg;
use common::*;
use sphere::Sphere;
use crate::hittable::{Hittable, Interval};
use crate::hittable_list::HittableList;

fn write_color(pixel_buffer: &mut [u8], color: Color) -> () {
    pixel_buffer[0] = (color[0] * 255.999) as u8;
    pixel_buffer[1] = (color[1] * 255.999) as u8;
    pixel_buffer[2] = (color[2] * 255.999) as u8;
}

fn ray_color(ray: &Ray, world: &impl Hittable) -> Color {
    match world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
        Some(hit_record) => {
            let n = &hit_record.normal.normalize();
            0.5 * Color::new(n[0] + 1.0, n[1] + 1.0, n[2] + 1.0)
        },
        None => {
            let unit_direction = &ray.direction.normalize();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let width: usize = 1080;
    let height: usize = ((width as f64) / aspect_ratio) as usize;
    let height: usize = height.max(1);

    // Camera
    let focal_length = 1.0f64;
    let viewport_height = 2.0f64;
    let viewport_width = viewport_height * (width as f64 / height as f64);
    let camera_center = nalgebra::Vector3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = nalgebra::Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = nalgebra::Vector3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / (width as f64);
    let pixel_delta_v = viewport_v / (height as f64);

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center
        - nalgebra::Vector3::new(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    

    // Create an array (Vec) of size width * height * 3 (e.g., RGB buffer)
    let size = width
        .checked_mul(height)
        .and_then(|px| px.checked_mul(3))
        .expect("width*height*3 overflowed");
    let mut buffer: Vec<u8> = vec![0u8; size];

    // Fill buffer per the C++ gradient: r = i/(w-1), g = j/(h-1), b = 0
    for j in 0..height {
        for i in 0..width {
            let pixel_center = pixel00_loc + (i as f64) * pixel_delta_u + (j as f64) * pixel_delta_v;
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let p = (j * width + i) * 3;
            write_color(
                &mut buffer[p..p+3],
                ray_color(&ray, &world)
            );
        }
    }

    match write_jpg("output/rgb_test.jpg", width, height, &buffer, 100) {
        Ok(()) => println!("Wrote rgb_test.jpg ({}x{} pixels)", width, height),
        Err(e) => eprintln!("Failed to write rgb_test.jpg: {}", e),
    }
}
