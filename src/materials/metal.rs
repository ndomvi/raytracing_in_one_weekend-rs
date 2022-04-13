use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::Point;

use rand::prelude::SmallRng;

#[derive(Debug)]
pub struct Metal {
    pub albedo: Point,
}

impl Metal {
    pub fn new(albedo: Point) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, _rng: &mut SmallRng) -> Option<(Ray, Point)> {
        let reflected = ray.reflected(rec);
        if reflected.direction.dot(rec.normal) > 0.0 {
            Some((reflected, self.albedo))
        } else {
            None
        }
    }
}
