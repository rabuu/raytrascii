use crate::color::Color;
use crate::hittable::HitRecord;
use crate::lalg::Vec3;
use crate::ray::Ray;

use super::Material;

#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    /// Default shorthand constructor
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_dir = rec.normal + Vec3::random_unit_vec();

        let scatter_dir = if scatter_dir.near_zero() {
            rec.normal
        } else {
            scatter_dir
        };

        let scattered = Ray::new(rec.p, scatter_dir);
        let attenuation = self.albedo;

        Some((attenuation, scattered))
    }

    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}
