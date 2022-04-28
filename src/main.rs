mod camera;
mod helpers;
mod hittable;
mod material;
mod materials;
mod objects;
mod ray;

use crate::camera::{ray_color, Camera};
use crate::hittable::HittableList;
use crate::material::Material;
use crate::materials::*;
use crate::objects::Sphere;

use glam::Vec3A;
use rand::distributions::Uniform;
use rand::prelude::*;
use rayon::prelude::*;
use std::fs::File;
use std::io::{stdout, BufWriter, Write};
use std::sync::Arc;
use std::time::Instant;

type Point = Vec3A;

fn main() {
    let start_t = Instant::now();

    println!("Started...");

    // Camera
    let look_from = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let vup = Point::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let focus_dist = 10.0;
    let aspect_ratio = 3.0 / 2.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        aperture,
        focus_dist,
        aspect_ratio,
    );

    // Image
    let image_w = 512;
    let image_h = (image_w as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // Scene
    let world = random_scene();

    // Output
    let outfile = File::create("out.ppm").unwrap();
    let mut outfile = BufWriter::new(outfile);

    // Render
    // Write file header
    write!(outfile, "P3\n{image_w} {image_h}\n255\n").unwrap();
    // Parallel iteration over rows
    let mut rows_done = 0;
    (0..image_h)
        .rev()
        .fold::<Vec<Point>, _>(Vec::new(), |mut arr, j| {
            print!("\r{rows_done}/{image_h}");
            rows_done += 1;
            stdout().flush().unwrap();
            (0..image_w)
                .into_par_iter()
                .map_init(
                    || {
                        // SmallRng is much (~30%) faster than thread_rng() in debug mode, and is slightly faster in release
                        // The "randomness" shouldn't really matter here, so the performance gain is probably worth it
                        SmallRng::from_entropy()
                        // thread_rng()
                    },
                    |rng, i: i32| {
                        let mut color = Point::ZERO;
                        for _ in 0..samples_per_pixel {
                            let u = (i as f32 + rng.gen::<f32>()) / (image_w - 1) as f32;
                            let v = (j as f32 + rng.gen::<f32>()) / (image_h - 1) as f32;

                            let ray = camera.get_ray(u, v, rng);
                            color += ray_color(&world, &ray, max_depth, rng);
                        }
                        color
                    },
                )
                .collect_into_vec(&mut arr);
            arr
        })
        .iter()
        .for_each(|pixel_value| write_pixel(&mut outfile, pixel_value, samples_per_pixel));

    outfile.flush().unwrap();
    println!("\rDone. Time taken: {}s", start_t.elapsed().as_secs_f32());
}

pub fn write_pixel(writer: &mut impl Write, color: &Point, samples_per_pixel: i32) {
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
    )
    .unwrap()
}

#[allow(dead_code)]
fn fixed_scene() -> HittableList {
    let mut world = HittableList::new();
    // Materials
    let mat_ground = Arc::new(Lambertian::new(Point::new(0.8, 0.8, 0.0)));
    let mat_center = Arc::new(Lambertian::new(Point::new(0.1, 0.2, 0.5)));
    // let left = Arc::new(Metal::new(Point::new(0.8, 0.8, 0.8), 0.3));
    let mat_left = Arc::new(Dielectric::new(1.5));
    let mat_left_inner = Arc::new(Dielectric::new(1.5));
    let mat_right = Arc::new(Metal::new(Point::new(0.8, 0.6, 0.2), 0.0));

    // Objects
    let sphere_ground = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, mat_ground);
    let sphere_center = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, mat_center);
    let sphere_left = Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, mat_left);
    let sphere_left_inner = Sphere::new(Point::new(-1.0, 0.0, -1.0), -0.3, mat_left_inner);
    let sphere_right = Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, mat_right);

    world.add(sphere_ground);
    world.add(sphere_center);
    world.add(sphere_left);
    world.add(sphere_left_inner);
    world.add(sphere_right);

    world
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_mat = Arc::new(Lambertian::new(Point::splat(0.5)));

    world.add(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_mat,
    ));

    let mut rng = thread_rng();
    for i in -11..11 {
        for j in -11..11 {
            let center = Point::new(
                i as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                j as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let rand_mat = rng.gen::<f32>();
                let mat: Arc<dyn Material>;
                if rand_mat < 0.85 {
                    let albedo =
                        Point::from(rng.gen::<[f32; 3]>()) * Point::from(rng.gen::<[f32; 3]>());
                    mat = Arc::new(Lambertian::new(albedo));
                } else if rand_mat < 0.95 {
                    let dist = Uniform::new_inclusive(0.5, 1.0);
                    let albedo = Point::new(
                        dist.sample(&mut rng),
                        dist.sample(&mut rng),
                        dist.sample(&mut rng),
                    );
                    let fuzz = rng.gen::<f32>();
                    mat = Arc::new(Metal::new(albedo, fuzz));
                } else {
                    mat = Arc::new(Dielectric::new(1.5));
                }

                world.add(Sphere::new(center, 0.2, mat));
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    world.add(Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, mat1));
    let mat2 = Arc::new(Lambertian::new(Point::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, mat2));
    let mat3 = Arc::new(Metal::new(Point::new(0.7, 0.6, 0.5), 0.0));
    world.add(Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, mat3));

    world
}
