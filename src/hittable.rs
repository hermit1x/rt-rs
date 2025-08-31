use crate::common::*;
use crate::ray::Ray;
pub use crate::interval::Interval;

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord>;
}