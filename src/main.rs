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
    Color, Vector3, Point3, unit_vector, dot_product, clamp, cross
};
use material::{Lambertian, Metal, Dielectric};

const IMAGE_BUFFER_SIZE: usize = 100 * 1024 * 1024;

const ASPECT_RATIO: f64 = 16.0 / 10.0;
const IMAGE_WIDTH: u64 = 1000;
const IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;
const VFOV: f64 = 50.0; // In degrees

const RAYS_PER_PIXEL: f64 = 1000.0;
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
    let material_center = Rc::new(Lambertian {albedo: Color::new(0.9, 0.1, 0.1)});
    let material_left = Rc::new(Metal::new(Color::new(0.5, 0.5, 0.5), 0.3));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    // Ground
    world.objects.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));

    // Center
    world.objects.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center)));

    // Left
    world.objects.push(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));

    // Right
    world.objects.push(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));

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

    let camera = Camera::new(
        Point3::new(0.0, 0.3, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Point3::new(0.0, 1.0, 0.0),
        VFOV, ASPECT_RATIO);
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
