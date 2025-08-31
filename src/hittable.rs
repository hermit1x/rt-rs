use std::sync::Arc;
use crate::common::*;
use crate::ray::Ray;
pub use crate::interval::Interval;
use crate::material::Material;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub material: Arc<dyn Material>,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord>;
}