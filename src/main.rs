mod camera;
mod ray;

use crate::camera::*;
use crate::ray::Ray;
use anyhow::Result;
use glam::Vec3;
use std::fs::File;
use std::io::{BufWriter, Write};

type Point = Vec3;

fn main() -> Result<()> {
    println!("Started...");

    // Image
    let aspect_ratio = 16f32 / 9f32;
    let image_w = 512;
    let image_h = (image_w as f32 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0f32;

    let origin = Point::new(0f32, 0f32, 0f32);
    let horizontal = Point::new(viewport_width, 0f32, 0f32);
    let vertical = Point::new(0f32, viewport_height, 0f32);

    let lower_left_corner =
        origin - horizontal / 2f32 - vertical / 2f32 - Point::new(0f32, 0f32, focal_length);

    // Output
    let outfile = File::create("out.ppm")?;
    let mut outfile = BufWriter::new(outfile);

    // Render
    write!(outfile, "P3\n{image_w} {image_h}\n255\n")?;
    for j in (0..image_h).rev() {
        for i in 0..image_w {
            let u = i as f32 / (image_w - 1) as f32;
            let v = j as f32 / (image_h - 1) as f32;

            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let pixel = ray_color(&ray);
            write_pixel(&mut outfile, &pixel)?;
        }
    }

    println!("Done.");
    Ok(())
}
