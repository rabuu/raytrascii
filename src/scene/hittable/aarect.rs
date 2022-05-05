//! Axis-aligned rectangles

use crate::lalg::{Point3, Vec3};
use crate::ray::Ray;
use crate::scene::material::Material;

use super::aabb::Aabb;
use super::{HitRecord, Hittable};

use self::AaRectPlaneCoords::*;

#[derive(Debug, Clone, Copy)]
pub enum AaRectPlaneCoords {
    Xy { x: (f64, f64), y: (f64, f64) },
    Xz { x: (f64, f64), z: (f64, f64) },
    Yz { y: (f64, f64), z: (f64, f64) },
}

/// A rectangle object
#[derive(Debug, Clone)]
pub struct AaRect {
    pub plane_coords: AaRectPlaneCoords,
    pub k: f64,
    pub mat_ptr: Box<dyn Material>,
}

impl AaRect {
    /// Default shorthand constructor
    pub fn new(plane_coords: AaRectPlaneCoords, k: f64, mat_ptr: Box<dyn Material>) -> Self {
        AaRect {
            plane_coords,
            k,
            mat_ptr,
        }
    }
}

impl Hittable for AaRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = match self.plane_coords {
            Xy { .. } => (self.k - ray.origin.z) / ray.dir.z,
            Xz { .. } => (self.k - ray.origin.y) / ray.dir.y,
            Yz { .. } => (self.k - ray.origin.x) / ray.dir.x,
        };

        if t < t_min || t > t_max {
            return None;
        }

        let (first, second) = match self.plane_coords {
            Xy { x, y } => (x, y),
            Xz { x, z } => (x, z),
            Yz { y, z } => (y, z),
        };

        let (f, s) = match self.plane_coords {
            Xy { .. } => (
                ray.origin.x + (t * ray.dir.x),
                ray.origin.y + (t * ray.dir.y),
            ),
            Xz { .. } => (
                ray.origin.x + (t * ray.dir.x),
                ray.origin.z + (t * ray.dir.z),
            ),
            Yz { .. } => (
                ray.origin.y + (t * ray.dir.y),
                ray.origin.z + (t * ray.dir.z),
            ),
        };

        if f < first.0 || f > first.1 || s < second.0 || s > second.1 {
            return None;
        }

        let outward_normal = match self.plane_coords {
            Xy { .. } => Vec3::new(0.0, 0.0, 1.0),
            Xz { .. } => Vec3::new(0.0, 1.0, 0.0),
            Yz { .. } => Vec3::new(1.0, 0.0, 0.0),
        };

        let p = ray.at(t);

        let hit = HitRecord::new_with_face_normal(p, self.mat_ptr.clone(), t, ray, outward_normal);

        Some(hit)
    }

    fn bounding_box(&self) -> Option<Aabb> {
        let k_min = self.k - 0.0001;
        let k_max = self.k + 0.0001;

        let (min, max) = match self.plane_coords {
            Xy { x, y } => (Point3::new(x.0, y.0, k_min), Point3::new(x.1, y.1, k_max)),
            Xz { x, z } => (Point3::new(x.0, k_min, z.0), Point3::new(x.1, k_max, z.1)),
            Yz { y, z } => (Point3::new(k_min, y.0, z.0), Point3::new(k_max, y.1, z.1)),
        };

        Some(Aabb::new(min, max))
    }
}
