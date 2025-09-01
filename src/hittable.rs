use crate::common::*;
pub use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use std::sync::Arc;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub material: Arc<dyn Material>,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord>;
}
