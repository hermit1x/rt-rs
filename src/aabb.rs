use crate::common::*;
use crate::interval::Interval;
use crate::ray::Ray;

#[derive(Debug)]
pub(crate) struct AABB {
    ix: Interval,
    iy: Interval,
    iz: Interval,
}

impl AABB {
    pub fn new(ix: Interval, iy: Interval, iz: Interval) -> Self {
        Self { ix, iy, iz }
    }

    pub fn from_point(a: &Point3, b: &Point3) -> Self {
        // Ensure intervals are ordered min..max on each axis
        let ix = if a.x < b.x { Interval::new(a.x, b.x) } else { Interval::new(b.x, a.x) };
        let iy = if a.y < b.y { Interval::new(a.y, b.y) } else { Interval::new(b.y, a.y) };
        let iz = if a.z < b.z { Interval::new(a.z, b.z) } else { Interval::new(b.z, a.z) };
        Self::new(ix, iy, iz)
    }

    pub fn merge(a: &AABB, b: &AABB) -> Self {
        Self::new(
            Interval::merge(&a.ix, &b.ix),
            Interval::merge(&a.iy, &b.iy),
            Interval::merge(&a.iz, &b.iz),
        )
    }

    pub fn axis_interval(&self, axis: usize) -> &Interval {
        if axis == 0 {
            &self.ix
        }
        else if axis == 1 {
            &self.iy
        }
        else if axis == 2 {
            &self.iz
        }
        else {
            panic!("axis_interval: invalid axis {} (expected 0, 1, or 2)", axis);
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> bool {
        let mut time_interval = *ray_t; // Interval is Copy
        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let dir = ray.direction[axis];
            // Handle near-zero direction to avoid division by zero; if ray is parallel, just check origin within slab
            if dir.abs() < 1e-12 {
                if ray.origin[axis] < ax.start || ray.origin[axis] > ax.end {
                    return false;
                }
                continue;
            }

            let inv_d = 1.0 / dir;
            let mut t0 = (ax.start - ray.origin[axis]) * inv_d;
            let mut t1 = (ax.end - ray.origin[axis]) * inv_d;

            if t0 > t1 {
                std::mem::swap(&mut t0, &mut t1);
            }

            if t0 > time_interval.start { time_interval.start = t0; }
            if t1 < time_interval.end { time_interval.end = t1; }

            if time_interval.start > time_interval.end {
                return false;
            }
        }
        true
    }
}