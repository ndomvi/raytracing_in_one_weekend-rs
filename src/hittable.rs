use crate::material::Material;
use crate::ray::Ray;
use crate::Point;

use std::sync::Arc;

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Debug)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Point,
    pub t: f32,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        hit_p: Point,
        mut outward_normal: Point,
        t: f32,
        ray: &Ray,
        material: Arc<dyn Material>,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        if !front_face {
            outward_normal = -outward_normal;
        }

        Self {
            p: hit_p,
            normal: outward_normal,
            t,
            front_face,
            material,
        }
    }
}

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    #[inline]
    pub fn add(&mut self, hittable: impl Hittable + 'static) {
        self.list.push(Box::new(hittable));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_t = t_max;
        let mut closest_hit = None;
        self.list.iter().for_each(|obj| {
            if let Some(hit_record) = obj.hit(ray, t_min, closest_t) {
                closest_t = hit_record.t;
                closest_hit = Some(hit_record);
            }
        });

        closest_hit
    }
}
