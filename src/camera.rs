use crate::helpers::*;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::Point;
use rand::prelude::SmallRng;

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Point,
    vertical: Point,
    lens_radius: f32,
    u: Point,
    v: Point,
}

impl Camera {
    pub fn new(
        look_from: Point,
        look_at: Point,
        vup: Point,
        vfov: f32,
        aperture: f32,
        focus_dist: f32,
        aspect_ratio: f32,
    ) -> Self {
        let viewport_height = 2.0 * (vfov.to_radians() / 2.0).tan();
        let viewport_width = viewport_height * aspect_ratio;

        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let h = focus_dist * viewport_width * u;
        let v = focus_dist * viewport_height * v;

        let lower_left_corner = look_from - h / 2.0 - v / 2.0 - focus_dist * w;
        let lens_radius = aperture / 2.0;

        Self {
            origin: look_from,
            lower_left_corner,
            horizontal: h,
            vertical: v,
            lens_radius,
            u,
            v,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32, rng: &mut SmallRng) -> Ray {
        let rand_disk = self.lens_radius * random_in_unit_disk(rng);
        let offset = self.u * rand_disk.x + self.v * rand_disk.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + (s * self.horizontal) + (t * self.vertical)
                - self.origin
                - offset,
        )
    }
}

pub fn ray_color(world: &impl Hittable, ray: &Ray, depth: i32, rng: &mut SmallRng) -> Point {
    if depth <= 0 {
        return Point::ZERO;
    }

    if let Some(rec) = world.hit(ray, 0.001, f32::INFINITY) {
        if let Some((scattered, attenuation)) = rec.material.scatter(ray, &rec, rng) {
            attenuation * ray_color(world, &scattered, depth - 1, rng)
        } else {
            Point::ZERO
        }
    } else {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Point::ONE + t * Point::new(0.5, 0.7, 1.0)
    }
}
