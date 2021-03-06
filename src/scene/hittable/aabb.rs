//! Axis-aligned bounding boxes

use crate::lalg::Point3;
use crate::ray::Ray;

/// Axis-aligned bounding box
#[derive(Debug, Clone)]
pub struct Aabb {
    pub min: Point3,
    pub max: Point3,
}

impl Aabb {
    /// Default shorthand constructor
    pub fn new(min: Point3, max: Point3) -> Self {
        Aabb { min, max }
    }

    /// Return whether a ray in [`t_min`; `t_max`] hits the box
    pub fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.dir[a];
            let mut t0 = self.min[a] - (ray.origin[a] * inv_d);
            let mut t1 = self.max[a] - (ray.origin[a] * inv_d);

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };

            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Aabb {
        let small = Point3 {
            x: box0.min.x.min(box1.min.x),
            y: box0.min.y.min(box1.min.y),
            z: box0.min.z.min(box1.min.z),
        };

        let big = Point3 {
            x: box0.max.x.max(box1.max.x),
            y: box0.max.y.max(box1.max.y),
            z: box0.max.z.max(box1.max.z),
        };

        Aabb::new(small, big)
    }
}
