mod ray;
mod write_img;
mod common;
mod sphere;
mod hittable;
mod hittable_list;
mod interval;
mod camera;
mod material;

use std::sync::Arc;
use write_img::write_jpg;
use common::*;
use sphere::Sphere;
use crate::hittable_list::HittableList;
use camera::Camera;
use crate::material::Material;

fn main() {
    // World
    let mut world = HittableList::new();

    let material_center: Arc<dyn Material> = Arc::new(material::Lambertian::new(Color::new(0.1, 0.2, 0.5)));

    world.add(Box::new(
        Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Arc::clone(&material_center))
    ));
    world.add(Box::new(
        Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Arc::clone(&material_center))
    ));

    // Camera
    let camera = Camera::new(16.0 / 9.0, 1920);
    let (width, height, buffer) = camera.render(&world);
    let file_name = "output/material.jpg";
    match write_jpg(file_name, width, height, &buffer, 100) {
        Ok(()) => println!("Wrote {} ({}x{} pixels)", file_name, width, height),
        Err(e) => eprintln!("Failed to write {}: {}", file_name, e),
    }
    return;
}
