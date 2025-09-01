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

    let material_ground: Arc<dyn Material> = Arc::new(material::Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(
        Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Arc::clone(&material_ground))
    ));

    let mut rng = rand::rngs::ThreadRng::default();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random(&mut rng);
            let center = Point3::new(a as f64 + 0.9 * random(&mut rng), 0.2, b as f64 + 0.9 * random(&mut rng));

            if (center - Point3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                let material: Arc<dyn Material>;

                if (choose_mat < 0.8) {
                    // diffuse
                    let albedo = Color::new(
                        random(&mut rng) * random(&mut rng),
                        random(&mut rng) * random(&mut rng),
                        random(&mut rng) * random(&mut rng)
                    );
                    material = Arc::new(material::Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, material)));
                }
                else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::new(
                        random_range(0.5, 1.0, &mut rng),
                        random_range(0.5, 1.0, &mut rng),
                        random_range(0.5, 1.0, &mut rng)
                    );
                    let fuzz = random_range(0.0, 0.5, &mut rng);
                    material = Arc::new(material::Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, material)));
                }
                else {
                    // glass
                    material = Arc::new(material::Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    let material_1: Arc<dyn Material> = Arc::new(material::Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, Arc::clone(&material_1))));

    let material_2: Arc<dyn Material> = Arc::new(material::Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, Arc::clone(&material_2))));

    let material_3: Arc<dyn Material> = Arc::new(material::Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, Arc::clone(&material_3))));

    // Camera
    let camera = Camera::new(16.0 / 9.0, 1920);
    let (width, height, buffer) = camera.render(&world);
    let file_name = "output/final_render.jpg";
    match write_jpg(file_name, width, height, &buffer, 100) {
        Ok(()) => println!("Wrote {} ({}x{} pixels)", file_name, width, height),
        Err(e) => eprintln!("Failed to write {}: {}", file_name, e),
    }
    return;
}
