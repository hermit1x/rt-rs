mod ray;
mod write_img;
mod common;
mod sphere;
mod hittable;
mod hittable_list;
mod interval;
mod camera;

use write_img::write_jpg;
use common::*;
use sphere::Sphere;
use crate::hittable_list::HittableList;
use camera::Camera;

fn main() {
    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new(16.0 / 9.0, 1920);
    let (width, height, buffer) = camera.render(&world);
    match write_jpg("output/rgb_test.jpg", width, height, &buffer, 100) {
        Ok(()) => println!("Wrote rgb_test.jpg ({}x{} pixels)", width, height),
        Err(e) => eprintln!("Failed to write rgb_test.jpg: {}", e),
    }
    return;
}
