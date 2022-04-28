use crate::helpers::random_in_unit_sphere;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::Point;

use rand::prelude::SmallRng;

#[derive(Debug)]
pub struct Metal {
    albedo: Point,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Point, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, rng: &mut SmallRng) -> Option<(Ray, Point)> {
        let mut reflected = ray.reflected(rec);
        reflected.direction += self.fuzz * random_in_unit_sphere(rng);

        if reflected.direction.dot(rec.normal) > 0.0 {
            Some((reflected, self.albedo))
        } else {
            None
        }
    }
}
