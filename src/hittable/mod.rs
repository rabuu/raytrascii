//! Hittable objects

use std::fmt::Debug;

use crate::lalg::{Point3, Vec3};
use crate::ray::Ray;

mod sphere;

pub use sphere::Sphere;

/// Trait for objects that can be hit
pub trait Hittable: Debug + Send + Sync {
    /// Function that indicates whether a [`Ray`] hits the object
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

/// Structure to store some information about a hit
#[derive(Debug, Clone, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub is_front_face: bool,
}

impl HitRecord {
    /// Default shorthand constructor
    pub fn new(p: Point3, normal: Vec3, t: f64, is_front_face: bool) -> Self {
        HitRecord {
            p,
            normal,
            t,
            is_front_face,
        }
    }

    /// Helper function to set normal
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.is_front_face = ray.dir.dot(outward_normal) < 0.0;

        self.normal = if self.is_front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

/// Container for objects that implement [`Hittable`]
#[derive(Debug)]
pub struct HittableList(Vec<Box<dyn Hittable>>);

impl HittableList {
    /// Default constructor
    pub fn new(objs: Vec<Box<dyn Hittable>>) -> Self {
        HittableList(objs)
    }

    /// Constructs an empty container
    pub fn empty() -> Self {
        HittableList(Vec::new())
    }

    /// Add an object to the container
    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.0.push(obj);
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::empty()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit = None;
        let mut closest = t_max;

        for obj in self.0.iter() {
            if let Some(rec) = obj.hit(ray, t_min, closest) {
                closest = rec.t;
                hit = Some(rec);
            }
        }

        hit
    }
}
