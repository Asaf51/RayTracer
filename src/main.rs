mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod material;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufWriter;
use rand::Rng;
use indicatif::{ProgressBar, ProgressStyle};
use std::rc::Rc;

use ray::Ray;
use hittable::{Hit, HittableList};
use sphere::Sphere;
use camera::Camera;
use vec3::{
    Color, Vector3, Point3, unit_vector, dot_product, clamp, cross, random_in_unit_disk
};
use material::{Lambertian, Metal, Dielectric};

const IMAGE_BUFFER_SIZE: usize = 100 * 1024 * 1024;

const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: u64 = 1200;
const IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;
const VFOV: f64 = 20.0; // In degrees

const RAYS_PER_PIXEL: f64 = 500.0;
const MAX_DEPTH: i8 = 50;

fn write_header(file: &mut BufWriter<File>) {
    writeln!(file, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT).unwrap();
}

fn write_color(file: &mut BufWriter<File>, color: &Color, samples: f64) {
    let scale = 1.0 / samples;

    let r = color.x * scale;
    let g = color.y * scale;
    let b = color.z * scale;

    writeln!(file, "{} {} {}",
        (clamp(r.sqrt(), 0.0, 0.999) * 256.0) as i32,
        (clamp(g.sqrt(), 0.0, 0.999) * 256.0) as i32,
        (clamp(b.sqrt(), 0.0, 0.999) * 256.0) as i32,
    ).unwrap();
}

fn ray_color(ray: &Ray, world: &HittableList, depth: i8) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = world.hit(ray, 0.001, std::f64::INFINITY) {
        if let Some(material_info) = hit.material.scatter(ray, &hit) {
            material_info.attenuation * ray_color(&material_info.scattered, world, depth - 1)
        } else {
            Color::default()
        }
    } else {
        // Get the normalized direction vector
        let unit_direction = unit_vector(&ray.direction);

        let t = 0.5 * (unit_direction.y + 1.0);
        let color_base = 1.0 - t;

        // color: 0.5, 0.7, 1.0
        Color::new(color_base + 0.5 * t, color_base + 0.7 * t, color_base + t)
    }
}

fn init_world() -> HittableList {
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian {albedo: Color::new(0.5, 0.5, 0.5)});
    world.objects.push(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, material_ground)));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let mat = rng.gen_range(0.0, 1.0);
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen_range(0.0, 1.0),
                0.2,
                b as f64 + 0.9 * rng.gen_range(0.0, 1.0)
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if mat < 0.8 {
                    let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                    let material = Rc::new(Lambertian {albedo});
                    world.objects.push(Box::new(Sphere::new(center, 0.2, material)));
                } else if mat < 0.95 {
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    let material = Rc::new(Metal::new(albedo, fuzz));
                    world.objects.push(Box::new(Sphere::new(center, 0.2, material)));
                } else {
                    let material = Rc::new(Dielectric::new(1.5));
                    world.objects.push(Box::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.objects.push(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Rc::new(Lambertian { albedo: Color::new(0.4, 0.2, 0.1) });
    world.objects.push(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.objects.push(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

    world
}

fn init_file(filename: &str) -> BufWriter<File> {
    let path = Path::new(filename);
    let file = match File::create(&path) {
        Err(why) => panic!("Couldn't create file {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut buf = BufWriter::with_capacity(IMAGE_BUFFER_SIZE, file);
    write_header(&mut buf);

    buf
}

fn main() {
    let mut buf = init_file("output1.ppm");
    let world = init_world();

    let lookfrom = Point3::new(13.0, 4.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);

    let camera = Camera::new(
        lookfrom,
        lookat,
        Point3::new(0.0, 1.0, 0.0),
        VFOV, ASPECT_RATIO, 0.1, 10.0);
    let mut rng = rand::thread_rng();

    let bar = ProgressBar::new(IMAGE_HEIGHT);
    bar.set_style(ProgressStyle::default_bar()
                 .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {percent}% ({eta_precise})")
                 .progress_chars("#>-"));

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..RAYS_PER_PIXEL as i32 {
                let u = ((i as f64) + rng.gen_range(0.0, 1.0)) / (IMAGE_WIDTH - 1) as f64;
                let v = ((j as f64) + rng.gen_range(0.0, 1.0)) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, MAX_DEPTH);
            }

            write_color(&mut buf, &pixel_color, RAYS_PER_PIXEL);
        }

        bar.inc(1);
    }

    bar.finish();

    buf.flush().unwrap();
}
