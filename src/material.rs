use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::Point;
use rand::prelude::SmallRng;

pub trait Material: std::fmt::Debug + Sync + Send {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, rng: &mut SmallRng) -> Option<(Ray, Point)>;
}
