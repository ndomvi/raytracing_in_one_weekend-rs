use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::Point;
use std::rc::Rc;

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f32, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;

        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();
        let mut root = (-half_b - discriminant_sqrt) / a;

        if (root < t_min) || (root > t_max) {
            root = (-half_b + discriminant_sqrt) / a;
            if (root < t_min) || (root > t_max) {
                return None;
            }
        }

        Some(HitRecord::new(
            ray.at(root),
            (ray.at(root) - self.center) / self.radius,
            root,
            ray,
            self.material.clone(),
        ))
    }
}
