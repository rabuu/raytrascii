use crate::lalg::{Point3, Vec3};
use crate::ray::Ray;

/// A static view on the scene
#[derive(Debug)]
pub(crate) struct CameraView {
    pub(crate) origin: Point3,
    pub(crate) lower_left_corner: Point3,
    pub(crate) horiz: Vec3,
    pub(crate) vert: Vec3,
}

impl CameraView {
    /// Return the ray located at a given point in the viewport
    pub(crate) fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray {
            origin: self.origin,
            dir: self.lower_left_corner + (s * self.horiz) + (t * self.vert) - self.origin,
        }
    }
}
