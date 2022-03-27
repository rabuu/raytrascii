use crate::lalg::Point3;
use crate::ray::Ray;

use super::{HitRecord, Hittable};

/// A sphere object
#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    /// Default shorthand constructor
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.dir.len_sq();
        let half_b = oc.dot(ray.dir);
        let c = oc.len_sq() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - (a * c);
        if discriminant < 0.0 {
            return None;
        }

        let mut root = (-half_b / discriminant.sqrt()) / a;
        if root < t_min || root > t_max {
            root = (-half_b + discriminant.sqrt()) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let mut hit = HitRecord::default();
        hit.t = root;
        hit.p = ray.at(hit.t);

        let outward_normal = (hit.p - self.center) / self.radius;
        hit.set_face_normal(ray, outward_normal);

        Some(hit)
    }
}
