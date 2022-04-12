use rand::prelude::SmallRng;

use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::{random_unit_vector, Point};

pub struct Camera {
    pub image_dim: (i32, i32),
    origin: Point,
    lower_left_corner: Point,
    horizontal: Point,
    vertical: Point,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        image_width: i32,
        viewport_height: f32,
        focal_length: f32,
    ) -> Self {
        // Image
        let image_w = image_width;
        let image_h = (image_w as f32 / aspect_ratio) as i32;

        // Camera
        let viewport_width = viewport_height * aspect_ratio;

        let origin = Point::new(0.0, 0.0, 0.0);
        let horizontal = Point::new(viewport_width, 0.0, 0.0);
        let vertical = Point::new(0.0, viewport_height, 0.0);

        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Point::new(0.0, 0.0, focal_length);
        Self {
            image_dim: (image_w, image_h),
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + (u * self.horizontal) + (v * self.vertical) - self.origin,
        )
    }
}

pub fn ray_color(world: &impl Hittable, ray: &Ray, depth: i32, rng: &mut SmallRng) -> Point {
    if depth <= 0 {
        return Point::ZERO;
    }

    match world.hit(ray, 0.001, f32::INFINITY) {
        Some(rec) => {
            let target = rec.p + rec.normal + random_unit_vector(rng);
            0.5 * ray_color(world, &Ray::new(rec.p, target - rec.p), depth - 1, rng)
        }
        None => {
            let unit_direction = ray.direction.normalize();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Point::ONE + t * Point::new(0.5, 0.7, 1.0)
        }
    }
}
