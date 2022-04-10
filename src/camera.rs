use crate::ray::Ray;
use anyhow::Result;
use glam::Vec3;
use std::io::Write;

type Point = Vec3;

pub(crate) fn write_pixel(writer: &mut impl Write, color: &Point) -> Result<()> {
    writeln!(
        writer,
        "{} {} {}",
        (color.x * 255f32) as i32,
        (color.y * 255f32) as i32,
        (color.z * 255f32) as i32
    )?;
    Ok(())
}

pub(crate) fn ray_color(ray: &Ray) -> Point {
    if hit_sphere(&Point::new(0f32, 0f32, -1f32), 0.5, ray) {
        Point::new(1f32, 0f32, 0f32)
    } else {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Point::ONE + t * Point::new(0.5, 0.7, 1.0)
    }
}

fn hit_sphere(center: &Point, radius: f32, ray: &Ray) -> bool {
    let oc = ray.origin - *center;

    let a = ray.direction.dot(ray.direction);
    let b = 2f32 * oc.dot(ray.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4f32 * a * c;

    discriminant > 0f32
}
