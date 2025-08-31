use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::common::{near_zero, random_unit_vec3, Color};
use rand::rngs::ThreadRng;

pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, rng: &mut ThreadRng) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    pub albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, rng: &mut ThreadRng) -> Option<(Ray, Color)> {
        let scatter_direction_sample = hit_record.normal + random_unit_vec3(rng);
        
        // catch degenerate scatter direction
        let scatter_direction = if near_zero(&scatter_direction_sample) { 
            hit_record.normal
        }
        else {
            scatter_direction_sample
        };
        
        let scattered = Ray::new(hit_record.point, scatter_direction);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}