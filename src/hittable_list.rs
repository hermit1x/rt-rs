use crate::hittable::{HitRecord, Hittable, Interval};
use crate::ray::Ray;
use crate::aabb::*;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable + Sync>>,
    pub aabb: AABB
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            aabb: AABB::new(
                Interval::EMPTY,
                Interval::EMPTY,
                Interval::EMPTY
            )
        }
    }
    pub fn add(&mut self, object: Box<dyn Hittable + Sync>) {
        self.aabb = AABB::merge(&self.aabb, object.get_aabb());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if !self.aabb.hit(ray, ray_t) {
            return None;
        }

        let mut closest_so_far = ray_t.end;
        let mut hit_record = None;
        for object in &self.objects {
            if let Some(record) = object.hit(ray, &Interval::new(ray_t.start, closest_so_far)) {
                closest_so_far = record.t;
                hit_record = Some(record);
            }
        }
        hit_record
    }

    fn get_aabb(&self) -> &AABB {
        &self.aabb
    }
}
