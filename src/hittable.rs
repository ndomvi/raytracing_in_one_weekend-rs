use crate::ray::Ray;
use glam::Vec3;

type Point = Vec3;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Debug)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Point,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(hit_p: Point, mut outward_normal: Point, t: f32, ray: &Ray) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        if !front_face {
            outward_normal = -outward_normal;
        }

        Self {
            p: hit_p,
            normal: outward_normal,
            t,
            front_face,
        }
    }
}
