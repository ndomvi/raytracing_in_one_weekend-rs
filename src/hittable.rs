use crate::ray::Ray;
use crate::Point;

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

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn add(&mut self, hittable: impl Hittable + 'static) {
        self.list.push(Box::new(hittable));
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_t = t_max;
        let mut closest_hit = None;
        for obj in &self.list {
            if let Some(hit_record) = obj.hit(ray, t_min, t_max) {
                if hit_record.t < closest_t {
                    closest_t = hit_record.t;
                    closest_hit = Some(hit_record);
                }
            }
        }

        closest_hit
    }
}
