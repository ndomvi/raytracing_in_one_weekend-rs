mod camera;
mod hittable;
mod hittables;
mod ray;

use crate::camera::*;
use crate::hittable::HittableList;
use crate::hittables::sphere::Sphere;
use anyhow::Result;
use glam::Vec3;
use rand::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

type Point = Vec3;

fn main() -> Result<()> {
    let start_t = Instant::now();
    // SmallRng is much (~30%) faster than thread_rng() in debug mode, and is slightly faster in release
    // The "randomness" shouldn't really matter here, so the performance gain is probably worth it
    let mut rng = SmallRng::from_entropy();
    // let mut rng = thread_rng();

    println!("Started...");

    // Camera and image
    let camera = Camera::new(16.0 / 9.0, 512, 2.0, 1.0);
    let (image_w, image_h) = camera.image_dim;
    let samples_per_pixel = 50;

    // Scene
    let mut world = HittableList::new();
    world.add(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0));

    // Output
    let outfile = File::create("out.ppm")?;
    let mut outfile = BufWriter::new(outfile);

    // Render
    write!(outfile, "P3\n{image_w} {image_h}\n255\n")?;
    for j in (0..image_h).rev() {
        for i in 0..image_w {
            let mut color = Point::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f32 + rng.gen::<f32>()) / (image_w - 1) as f32;
                let v = (j as f32 + rng.gen::<f32>()) / (image_h - 1) as f32;
                let ray = camera.get_ray(u, v);
                color += ray_color(&world, &ray);
            }
            write_pixel(&mut outfile, &color, samples_per_pixel)?;
        }
    }

    println!("Done. Time taken: {}s", start_t.elapsed().as_secs_f32());
    Ok(())
}

pub fn write_pixel(writer: &mut impl Write, color: &Point, samples_per_pixel: i32) -> Result<()> {
    let [mut r, mut g, mut b] = color.to_array();

    let samples_per_pixel = samples_per_pixel as f32;
    r /= samples_per_pixel;
    g /= samples_per_pixel;
    b /= samples_per_pixel;

    writeln!(
        writer,
        "{} {} {}",
        (r.clamp(0.0, 0.9999) * 256.0) as i32,
        (g.clamp(0.0, 0.9999) * 256.0) as i32,
        (b.clamp(0.0, 0.9999) * 256.0) as i32
    )?;
    Ok(())
}
