//! Computation of rays

use crate::lalg::{Point3, Vec3};

/// Ray structure that stores an origin a direction
#[derive(Debug, Clone, Default)]
pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
}

impl Ray {
    /// Default shorthand constructor
    pub fn new(origin: Point3, dir: Vec3) -> Self {
        Ray { origin, dir }
    }

    /// Return the [point][Point3] located at distance `d` in the ray direction from the origin
    pub fn at(&self, d: f64) -> Point3 {
        self.origin + (d * self.dir)
    }
}
