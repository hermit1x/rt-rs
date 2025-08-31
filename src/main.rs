mod ray;

mod write_img;
mod common;
mod sphere;
mod hittable;
mod hittable_list;
mod interval;
mod camera;

use ray::Ray;
use write_img::write_jpg;
use common::*;
use sphere::Sphere;
use crate::hittable::{Hittable, Interval};
use crate::hittable_list::HittableList;
use camera::Camera;

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

    let camera = Camera::new(16.0 / 9.0, 1080);
    let (width, height, buffer) = camera.render(&world);
    match write_jpg("output/rgb_test.jpg", width, height, &buffer, 100) {
        Ok(()) => println!("Wrote rgb_test.jpg ({}x{} pixels)", width, height),
        Err(e) => eprintln!("Failed to write rgb_test.jpg: {}", e),
    }
    return;
}
