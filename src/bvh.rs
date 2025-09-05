use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable, Interval};
use crate::interval::Interval as AxisInterval;
use crate::ray::Ray;
use std::sync::Arc;

pub struct BvhNode {
    pub aabb: AABB,
    pub left: Arc<dyn Hittable + Send + Sync>,
    pub right: Arc<dyn Hittable + Send + Sync>,
}

impl BvhNode {
    pub fn from_list(mut list: Vec<Box<dyn Hittable + Send + Sync>>) -> Arc<BvhNode> {
        assert!(!list.is_empty(), "List must not be empty!");
        if list.len() == 1 {
            // Create a degenerate node with both children being the same object
            let obj: Arc<dyn Hittable + Send + Sync> = Arc::from(list.remove(0));
            let aabb = obj.get_aabb();
            let node = BvhNode {
                aabb: AABB::new(*aabb.axis_interval(0), *aabb.axis_interval(1), *aabb.axis_interval(2)),
                left: Arc::clone(&obj),
                right: obj,
            };
            return Arc::new(node);
        }

        // Compute the overall bounding box for the list
        let mut global = AABB::new(AxisInterval::EMPTY, AxisInterval::EMPTY, AxisInterval::EMPTY);
        for obj in &list {
            global = AABB::merge(&global, obj.get_aabb());
        }

        // Determine the longest axis: 0=x,1=y,2=z
        let x_len = global.axis_interval(0).end - global.axis_interval(0).start;
        let y_len = global.axis_interval(1).end - global.axis_interval(1).start;
        let z_len = global.axis_interval(2).end - global.axis_interval(2).start;
        let split_axis = if x_len >= y_len && x_len >= z_len {
            0
        } else if y_len >= z_len {
            1
        } else {
            2
        };

        // Sort along the chosen axis using the AABB centers
        list.sort_by(|a, b| {
            let aa = a.get_aabb().axis_interval(split_axis);
            let ba = b.get_aabb().axis_interval(split_axis);
            let ac = 0.5 * (aa.start + aa.end);
            let bc = 0.5 * (ba.start + ba.end);
            ac.partial_cmp(&bc).unwrap_or(std::cmp::Ordering::Equal)
        });

        let mid = list.len() / 2;
        let right_list = list.split_off(mid);
        let left_list = list; // the remaining left half

        let left = BvhNode::from_list(left_list);
        let right = BvhNode::from_list(right_list);
        let aabb = AABB::merge(left.get_aabb(), right.get_aabb());

        Arc::new(BvhNode {
            aabb,
            left: left as Arc<dyn Hittable + Send + Sync>,
            right: right as Arc<dyn Hittable + Send + Sync>
        })
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        if !self.aabb.hit(ray, interval) {
            return None;
        }
        let record_left = self.left.hit(ray, interval);
        let record_right = self.right.hit(ray, interval);

        match (record_left, record_right) {
            (Some(l), Some(r)) => {
                if l.t <= r.t { Some(l) } else { Some(r) }
            }
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            (None, None) => None,
        }
    }

    fn get_aabb(&self) -> &AABB { &self.aabb }
}