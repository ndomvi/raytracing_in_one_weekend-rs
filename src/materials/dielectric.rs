use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::Point;

use rand::prelude::SmallRng;

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
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, _rng: &mut SmallRng) -> Option<(Ray, Point)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let cos_theta = (-ray.direction.normalize()).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let scattered = if refraction_ratio * sin_theta > 1.0 {
            ray.reflected(rec)
        } else {
            ray.refracted(rec, refraction_ratio)
        };

        Some((scattered, Point::ONE))
    }
}
