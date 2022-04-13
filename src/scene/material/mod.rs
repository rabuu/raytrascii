//! Object materials

use std::fmt::Debug;

use crate::color::Color;
use crate::ray::Ray;
use crate::scene::hittable::HitRecord;

mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

/// Material that defines how an object interacts with its environment
pub trait Material: Debug + Send + Sync {
    /// Provide the way how the material handles incoming rays
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;

    /// Provide a method for cloning as trait object
    fn box_clone(&self) -> Box<dyn Material>;
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

/// Default non-scattering material
#[derive(Debug, Clone)]
pub struct DefaultMaterial;

impl DefaultMaterial {
    /// Wrap in a `Box`
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Material for DefaultMaterial {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
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
