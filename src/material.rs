use crate::common::{near_zero, random, random_unit_vec3, reflect, refract, Color};
use crate::hittable::HitRecord;
use crate::ray::Ray;
use rand::rngs::ThreadRng;

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        hit_record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, Color)> {
        let scatter_direction_sample = hit_record.normal + random_unit_vec3(rng);

        // catch degenerate scatter direction
        let scatter_direction = if near_zero(&scatter_direction_sample) {
            hit_record.normal
        } else {
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
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, Color)> {
        let reflected =
            reflect(&ray_in.direction, &hit_record.normal) + self.fuzz * random_unit_vec3(rng);
        let scattered = Ray::new(hit_record.point, reflected);
        let attenuation = self.albedo;
        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(&self, cosine: f64, reflection_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = (1.0 - reflection_index) / (1.0 + reflection_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let reflection_index = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction;
        let cos_theta = f64::min(-unit_direction.dot(&hit_record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = reflection_index * sin_theta > 1.0;
        let direction =
            if cannot_refract || self.reflectance(cos_theta, reflection_index) > random(rng) {
                reflect(&unit_direction, &hit_record.normal)
            } else {
                refract(&unit_direction, &hit_record.normal, reflection_index)
            };

        let scattered = Ray::new(hit_record.point, direction);
        Some((scattered, attenuation))
    }
}
