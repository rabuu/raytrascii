//! Axis-aligned rectangles

use crate::lalg::{Point3, Vec3};
use crate::ray::Ray;
use crate::scene::material::Material;

use super::aabb::Aabb;
use super::{HitRecord, Hittable};

/// A XY-plane rectangle object
#[derive(Debug, Clone)]
pub struct XyRect {
    pub x: (f64, f64),
    pub y: (f64, f64),
    pub k: f64,
    pub mat_ptr: Box<dyn Material>,
}

impl XyRect {
    /// Default shorthand constructor
    pub fn new(x: (f64, f64), y: (f64, f64), k: f64, mat_ptr: Box<dyn Material>) -> Self {
        XyRect { x, y, k, mat_ptr }
    }
}

impl Hittable for XyRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z) / ray.dir.z;

        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin.x + (t * ray.dir.x);
        let y = ray.origin.y + (t * ray.dir.y);

        if x < self.x.0 || x > self.x.1 || y < self.y.0 || y > self.y.1 {
            return None;
        }

        // let u = (x - self.x.0) / (self.x.1 - self.x.0);
        // let v = (y - self.y.0) / (self.y.1 - self.y.0);
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        let p = ray.at(t);

        let hit = HitRecord::new_with_face_normal(p, self.mat_ptr.clone(), t, ray, outward_normal);

        Some(hit)
    }

    fn bounding_box(&self) -> Option<Aabb> {
        let aabb = Aabb {
            min: Point3::new(self.x.0, self.y.0, self.k - 0.0001),
            max: Point3::new(self.x.1, self.y.1, self.k + 0.0001),
        };

        Some(aabb)
    }
}
