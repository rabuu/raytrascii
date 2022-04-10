use crate::color::Color;
use crate::ray::Ray;
use crate::scene::hittable::HitRecord;

use super::Material;

#[derive(Debug, Clone)]
pub struct Dielectric {
    pub ir: f64, // index of refraction
}

impl Dielectric {
    /// Default shorthand constructor
    pub fn new(index_of_refraction: f64) -> Self {
        Dielectric {
            ir: index_of_refraction,
        }
    }

    /// Wrap in a `Box`
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // use Schlick's approximation for reflectance
        let r0_sqrt = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0_sqrt.powi(2);

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::white();

        let refraction_ratio = if rec.is_front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let cos_theta = (-ray_in.dir.unit_vec()).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > rand::random()
        {
            ray_in.dir.unit_vec().reflect(rec.normal)
        } else {
            ray_in.dir.unit_vec().refract(rec.normal, refraction_ratio)
        };

        let scattered = Ray::new(rec.p, direction);

        Some((attenuation, scattered))
    }

    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}
