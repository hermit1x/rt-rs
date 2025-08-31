use crate::common::*;
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Solve |(ray.origin + t*ray.direction) - center|^2 = radius^2
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let half_b = oc.dot(&ray.direction); // since equation is a t^2 + 2*b t + c, use half_b = b
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;
        // Determine front_face and adjust normal to always oppose the ray direction
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };

        Some(HitRecord { point, normal, t: root })
    }
}