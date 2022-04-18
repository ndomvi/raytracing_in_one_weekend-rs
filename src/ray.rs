use crate::hittable::HitRecord;
use crate::Point;

pub struct Ray {
    pub origin: Point,
    pub direction: Point,
}

impl Ray {
    pub fn new(origin: Point, direction: Point) -> Self {
        Self { origin, direction }
    }

    #[inline]
    pub fn at(&self, t: f32) -> Point {
        self.origin + t * self.direction
    }

    #[inline]
    pub fn reflected(&self, rec: &HitRecord) -> Self {
        let v = self.direction.normalize();
        let n = rec.normal;

        Self {
            origin: rec.p,
            direction: v - 2.0 * v.dot(n) * n,
        }
    }

    #[inline]
    pub fn refracted(&self, rec: &HitRecord, refraction_ratio: f32) -> Self {
        let v = self.direction.normalize();

        let cos_theta = (-v).dot(rec.normal).min(1.0);

        let ray_perpendicular = refraction_ratio * (v + cos_theta * rec.normal);
        let ray_parallel = -((1.0 - ray_perpendicular.length_squared()).sqrt()) * rec.normal;

        Self {
            origin: rec.p,
            direction: ray_perpendicular + ray_parallel,
        }
    }
}
