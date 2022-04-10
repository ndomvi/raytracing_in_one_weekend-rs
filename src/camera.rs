use crate::ray::Ray;
use anyhow::Result;
use glam::Vec3;
use std::io::Write;

type Point = Vec3;

pub(crate) fn write_pixel(writer: &mut impl Write, color: &Point) -> Result<()> {
    writeln!(
        writer,
        "{} {} {}",
        (color.x * 255.0) as i32,
        (color.y * 255.0) as i32,
        (color.z * 255.0) as i32
    )?;
    Ok(())
}

pub(crate) fn ray_color(ray: &Ray) -> Point {
    let t = hit_sphere(&Point::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let n = (ray.at(t) - Point::new(0.0, 0.0, -1.0)).normalize();
        0.5 * Point::new(n.x + 1.0, n.y + 1.0, n.z + 1.0)
    } else {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Point::ONE + t * Point::new(0.5, 0.7, 1.0)
    }
}

fn hit_sphere(center: &Point, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin - *center;

    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}
