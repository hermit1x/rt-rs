use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::common::{near_zero, random_unit_vec3, reflect, Color};
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

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, _fuzz: f64) -> Self {
        let fuzz = if _fuzz < 1.0 { _fuzz } else { 1.0 };
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, rng: &mut ThreadRng) -> Option<(Ray, Color)> {
        let reflected = reflect(&ray_in.direction, &hit_record.normal) + self.fuzz * random_unit_vec3(rng);
        let scattered = Ray::new(hit_record.point, reflected);
        let attenuation = self.albedo;
        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            Some((scattered, attenuation))
        }
        else {
            None
        }
    }
}