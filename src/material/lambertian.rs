use crate::brightness::Brightness;
use crate::hittable::HitRecord;
use crate::lalg::Vec3;
use crate::ray::Ray;

use super::Material;

#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo: Brightness,
}

impl Lambertian {
    /// Default shorthand constructor
    pub fn new(albedo: Brightness) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<(Brightness, Ray)> {
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
