use crate::hittable::{HitRecord, Hittable, Interval};
use crate::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable + Sync>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, object: Box<dyn Hittable + Sync>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.end;
        let mut hit_record = None;
        for object in &self.objects {
            if let Some(record) = object.hit(ray, Interval::new(ray_t.start, closest_so_far)) {
                closest_so_far = record.t;
                hit_record = Some(record);
            }
        }
        hit_record
    }
}
