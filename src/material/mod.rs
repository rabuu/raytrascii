use std::fmt::Debug;

use crate::brightness::Brightness;
use crate::hittable::HitRecord;
use crate::ray::Ray;

mod lambertian;

pub use lambertian::Lambertian;

/// Material that defines how an object interacts with its environment
pub trait Material: Debug + Send + Sync {
    /// Provide the way how the material handles incoming rays
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Brightness, Ray)>;

    /// Provide a method for cloning as trait object
    fn box_clone(&self) -> Box<dyn Material>;
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

#[derive(Debug, Clone)]
pub struct DefaultMaterial;

impl Material for DefaultMaterial {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<(Brightness, Ray)> {
        None
    }

    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

impl Default for Box<dyn Material> {
    fn default() -> Self {
        Box::new(DefaultMaterial)
    }
}
