mod ray;
mod write_img;
mod common;
mod sphere;
mod hittable;
mod hittable_list;
mod interval;
mod camera;
mod material;

use std::f64::consts::PI;
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

    let material_ground: Arc<dyn Material> = Arc::new(material::Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center: Arc<dyn Material> = Arc::new(material::Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left: Arc<dyn Material> = Arc::new(material::Dielectric::new(1.5));
    let material_bubble: Arc<dyn Material> = Arc::new(material::Dielectric::new(1.0 / 1.5));
    let material_right: Arc<dyn Material> = Arc::new(material::Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(
        Sphere::new(Point3::new(0.0, -100.5, 0.0), 100.0, Arc::clone(&material_ground))
    ));
    world.add(Box::new(
        Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, Arc::clone(&material_center))
    ));
    world.add(Box::new(
        Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Arc::clone(&material_left))
    ));
    world.add(Box::new(
        Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, Arc::clone(&material_bubble))
    ));
    world.add(Box::new(
        Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Arc::clone(&material_right))
    ));

    // Camera
    let camera = Camera::new(16.0 / 9.0, 1920);
    let (width, height, buffer) = camera.render(&world);
    let file_name = "output/camera_fov.jpg";
    match write_jpg(file_name, width, height, &buffer, 100) {
        Ok(()) => println!("Wrote {} ({}x{} pixels)", file_name, width, height),
        Err(e) => eprintln!("Failed to write {}: {}", file_name, e),
    }
    return;
}
