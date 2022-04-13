use crate::{hittable::HitRecord, Point};

pub struct Ray {
    pub origin: Point,
    pub direction: Point,
}

impl Ray {
    pub fn new(origin: Point, direction: Point) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Point {
        self.origin + t * self.direction
    }

    pub fn reflected(&self, rec: &HitRecord) -> Self {
        let v = self.direction.normalize();
        let n = rec.normal;

        Self {
            origin: rec.p,
            direction: v - 2.0 * v.dot(n) * n,
        }
    }
}
