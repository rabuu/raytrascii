use crate::lalg::Point3;
use crate::ray::Ray;
use crate::scene::material::Material;

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

    /// Construct a Sphere using a builder
    pub fn builder() -> SphereCenterBuilder {
        SphereCenterBuilder
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

pub struct SphereCenterBuilder;

impl SphereCenterBuilder {
    pub fn center(self, center: Point3) -> SphereRadiusBuilder {
        SphereRadiusBuilder { center }
    }

    pub fn center_xyz(self, x: f64, y: f64, z: f64) -> SphereRadiusBuilder {
        SphereRadiusBuilder {
            center: Point3::new(x, y, z),
        }
    }
}

pub struct SphereRadiusBuilder {
    center: Point3,
}

impl SphereRadiusBuilder {
    pub fn radius(self, radius: f64) -> SphereMaterialBuilder {
        SphereMaterialBuilder {
            center: self.center,
            radius,
        }
    }
}

pub struct SphereMaterialBuilder {
    center: Point3,
    radius: f64,
}

impl SphereMaterialBuilder {
    pub fn material(self, mat_ptr: Box<dyn Material>) -> SphereBuilder {
        SphereBuilder {
            center: self.center,
            radius: self.radius,
            mat_ptr,
        }
    }
}

pub struct SphereBuilder {
    center: Point3,
    radius: f64,
    mat_ptr: Box<dyn Material>,
}

impl SphereBuilder {
    pub fn build(self) -> Sphere {
        Sphere {
            center: self.center,
            radius: self.radius,
            mat_ptr: self.mat_ptr,
        }
    }

    pub fn build_boxed(self) -> Box<Sphere> {
        Box::new(self.build())
    }
}
