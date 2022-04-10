use crate::color::Color;
use crate::hittable::HitRecord;
use crate::lalg::Vec3;
use crate::ray::Ray;

use super::Material;

#[derive(Debug, Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    /// Default shorthand constructor
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray_in.dir.unit_vec().reflect(rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        let attenuation = self.albedo;

        if scattered.dir.dot(rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }

    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}
