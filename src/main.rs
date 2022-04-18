mod camera;
mod hittable;
mod material;
mod materials;
mod objects;
mod ray;

use crate::camera::*;
use crate::hittable::HittableList;
use crate::materials::*;
use crate::objects::*;

use anyhow::Result;
use glam::Vec3A;
use rand::distributions::Uniform;
use rand::prelude::*;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::Arc;
use std::time::Instant;

type Point = Vec3A;

fn main() -> Result<()> {
    let start_t = Instant::now();

    println!("Started...");

    // Camera and image
    let camera = Camera::new(16.0 / 9.0, 512, 2.0, 1.0);
    let (image_w, image_h) = camera.image_dim;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // Scene
    let mut world = HittableList::new();

    // Materials
    let ground = Arc::new(Lambertian::new(Point::new(0.8, 0.8, 0.0)));
    let center = Arc::new(Lambertian::new(Point::new(0.1, 0.2, 0.5)));
    // let left = Arc::new(Metal::new(Point::new(0.8, 0.8, 0.8), 0.3));
    let left = Arc::new(Dielectric::new(1.5));
    let right = Arc::new(Metal::new(Point::new(0.8, 0.6, 0.2), 0.0));

    // Objects
    world.add(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, ground));
    world.add(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, center));
    world.add(Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, left));
    world.add(Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, right));

    // Output
    let outfile = File::create("out.ppm")?;
    let mut outfile = BufWriter::new(outfile);

    // Render
    write!(outfile, "P3\n{image_w} {image_h}\n255\n")?;
    let pixel_values = (0..image_h)
        .rev()
        .collect::<Vec<i32>>()
        .into_par_iter()
        .map_init(
            || {
                // SmallRng is much (~30%) faster than thread_rng() in debug mode, and is slightly faster in release
                // The "randomness" shouldn't really matter here, so the performance gain is probably worth it
                SmallRng::from_entropy()
                // thread_rng()
            },
            |rng, j| -> Vec<Point> {
                (0..image_w)
                    .map(|i| {
                        let mut color = Point::new(0.0, 0.0, 0.0);
                        for _ in 0..samples_per_pixel {
                            let u = (i as f32 + rng.gen::<f32>()) / (image_w - 1) as f32;
                            let v = (j as f32 + rng.gen::<f32>()) / (image_h - 1) as f32;

                            let ray = camera.get_ray(u, v);
                            color += ray_color(&world, &ray, max_depth, rng);
                        }
                        color
                    })
                    .collect()
            },
        )
        .collect::<Vec<Vec<Point>>>();

    pixel_values
        .iter()
        .flatten()
        .try_for_each(|pixel_value| -> Result<()> {
            write_pixel(&mut outfile, pixel_value, samples_per_pixel)
        })?;

    outfile.flush()?;
    println!("Done. Time taken: {}s", start_t.elapsed().as_secs_f32());
    Ok(())
}

pub fn write_pixel(writer: &mut impl Write, color: &Point, samples_per_pixel: i32) -> Result<()> {
    let [mut r, mut g, mut b] = color.to_array();

    let scale = 1.0 / samples_per_pixel as f32;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    writeln!(
        writer,
        "{} {} {}",
        (r * 255.0) as i32,
        (g * 255.0) as i32,
        (b * 255.0) as i32
    )?;
    Ok(())
}

#[inline]
fn random_in_unit_sphere(rng: &mut SmallRng) -> Point {
    let distr = Uniform::new_inclusive(-1.0, 1.0);

    loop {
        let v = Point::new(distr.sample(rng), distr.sample(rng), distr.sample(rng));

        if v.length_squared() < 1.0 {
            return v;
        }
    }
}

#[inline]
fn random_unit_vector(rng: &mut SmallRng) -> Point {
    random_in_unit_sphere(rng).normalize()
}
