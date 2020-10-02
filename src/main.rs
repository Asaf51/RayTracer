mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufWriter;
use rand::Rng;
use indicatif::{ProgressBar, ProgressStyle};

use ray::Ray;
use hittable::{Hit, HittableList};
use sphere::Sphere;
use camera::Camera;
use vec3::{
    Color, Vector3, Point3, unit_vector, dot_product, clamp
};

const ASPECT_RATIO: f64 = 16.0 / 10.0;
const IMAGE_WIDTH: u64 = 1920;
const IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;

const RAYS_PER_PIXEL: f64 = 100.0;
const MAX_DEPTH: i8 = 50;

fn write_header(file: &mut BufWriter<File>) {
    writeln!(file, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
}

fn write_color(file: &mut BufWriter<File>, color: &Color, samples: f64) {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    let scale = 1.0 / samples;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    writeln!(file, "{} {} {}", 
        (clamp(r, 0.0, 0.999) * 256.0) as i32,
        (clamp(g, 0.0, 0.999) * 256.0) as i32,
        (clamp(b, 0.0, 0.999) * 256.0) as i32,
    );
}

fn ray_color(ray: &Ray, world: &HittableList, depth: i8) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let hit = world.hit(ray, 0.001, std::f64::INFINITY);
    if hit.is_some() {
        let hit_record = hit.unwrap();
        let target = hit_record.point + hit_record.normal + Vector3::new_random_unit();
        return ray_color(&Ray::new(hit_record.point, target - hit_record.point), world, depth - 1) * 0.5;
    }

    // Get the normalized direction vector 
    let unit_direction = unit_vector(&ray.direction);

    let t = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn init_world() -> HittableList {
    let mut world = HittableList::new();
    world.objects.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.objects.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    world
}

fn init_file(filename: &str) -> BufWriter<File> {
    let path = Path::new(filename);
    let file = match File::create(&path) {
        Err(why) => panic!("Couldn't create file {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut buf = BufWriter::new(file);
    write_header(&mut buf);

    buf
}

fn main() {
    let mut buf = init_file("output1.ppm");
    let world = init_world();

    let camera = Camera::new();
    let mut rng = rand::thread_rng();

    let bar = ProgressBar::new(IMAGE_HEIGHT);
    bar.set_style(ProgressStyle::default_bar()
                 .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {percent}% ({eta_precise})")
                 .progress_chars("#>-"));

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for s in 0..RAYS_PER_PIXEL as i32 {
                let u = ((i as f64) + rng.gen_range(0.0, 1.0))/ (IMAGE_WIDTH - 1) as f64;
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
