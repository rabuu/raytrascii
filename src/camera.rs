use crate::lalg::{OrthNormBasis3, Point3, Vec3};
use crate::ray::Ray;
use crate::utils;

/// Virtual camera structure
#[derive(Debug)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horiz: Vec3,
    vert: Vec3,
}

impl Camera {
    /// Default constructor
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let theta = utils::degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let origin = lookfrom;
        let orientation = OrthNormBasis3::orientation(lookfrom, lookat, vup);

        let horiz = viewport_width * orientation.u;
        let vert = viewport_height * orientation.v;
        let lower_left_corner = origin - (horiz / 2.0) - (vert / 2.0) - orientation.w;

        Camera {
            origin,
            lower_left_corner,
            horiz,
            vert,
        }
    }

    /// Return the [Ray] located at a given point in the viewport
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray {
            origin: self.origin,
            dir: self.lower_left_corner + (s * self.horiz) + (t * self.vert) - self.origin,
        }
    }
}
