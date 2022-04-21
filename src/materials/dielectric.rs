use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::Point;

use rand::prelude::SmallRng;
use rand::Rng;

#[derive(Debug)]
pub struct Dielectric {
    ir: f32,
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }

    fn reflectance(cosine: f32, refraction_ratio: f32) -> f32 {
        let r0 = ((1.0 - refraction_ratio) / (1.0 + refraction_ratio)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, rng: &mut SmallRng) -> Option<(Ray, Point)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let cos_theta = (-ray.direction.normalize()).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let scattered = if (refraction_ratio * sin_theta > 1.0)
            || (Self::reflectance(cos_theta, refraction_ratio) > rng.gen::<f32>())
        {
            ray.reflected(rec)
        } else {
            ray.refracted(rec, refraction_ratio)
        };

        Some((scattered, Point::ONE))
    }
}
