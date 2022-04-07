use crate::lalg::Point3;
use crate::material::Material;
use crate::ray::Ray;

use super::{HitRecord, Hittable};

/// A sphere object
#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Box<dyn Material>,
}

impl Sphere {
    /// Default shorthand constructor
    pub fn new(center: Point3, radius: f64, mat_ptr: Box<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            mat_ptr,
        }
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

        let mut root = (-half_b - discriminant.sqrt()) / a;
        if root < t_min || t_max < root {
            root = (-half_b + discriminant.sqrt()) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = ray.at(root);
        let outward_normal = (p - self.center) / self.radius;

        let hit =
            HitRecord::new_with_face_normal(p, self.mat_ptr.clone(), root, ray, outward_normal);

        Some(hit)
    }
}
