use crate::helpers::random_unit_vector;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::Point;

use rand::prelude::SmallRng;

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Point,
}

impl Lambertian {
    pub fn new(albedo: Point) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord, rng: &mut SmallRng) -> Option<(Ray, Point)> {
        let mut direction = rec.normal + random_unit_vector(rng);
        // Avoid bounces close to zero
        if direction.x < 1e-8 && direction.y < 1e-8 && direction.z < 1e-8 {
            direction = rec.normal;
        }
        Some((Ray::new(rec.p, direction), self.albedo))
    }
}
