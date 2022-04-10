use glam::Vec3;

type Point = Vec3;

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
}
