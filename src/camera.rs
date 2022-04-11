use crate::{hittable::Hittable, hittables::sphere::Sphere, ray::Ray};

use anyhow::Result;
use glam::Vec3;
use std::io::Write;

type Point = Vec3;

pub fn write_pixel(writer: &mut impl Write, color: &Point) -> Result<()> {
    writeln!(
        writer,
        "{} {} {}",
        (color.x * 255.0) as i32,
        (color.y * 255.0) as i32,
        (color.z * 255.0) as i32
    )?;
    Ok(())
}

pub fn ray_color(ray: &Ray) -> Point {
    let sphere = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5);
    let rec = sphere.hit(ray, -100.0, 100.0);
    match rec {
        Some(rec) => {
            let n = (ray.at(rec.t) - Point::new(0.0, 0.0, -1.0)).normalize();
            0.5 * Point::new(n.x + 1.0, n.y + 1.0, n.z + 1.0)
        }
        None => {
            let unit_direction = ray.direction.normalize();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Point::ONE + t * Point::new(0.5, 0.7, 1.0)
        }
    }
}
