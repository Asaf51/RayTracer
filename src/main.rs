mod vec3;
mod ray;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufWriter;

use ray::Ray;
use vec3::{
    Color, Vector3, Point3, unit_vector, dot_product
};

const ASPECT_RATIO: f64 = 16.0 / 10.0;
const IMAGE_WIDTH: i32 = 1920;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

// (0, 0, 0)
const CAMERA_ORIGIN: Point3 = Point3 {x: 0.0, y: 0.0, z: 0.0};
const HORIZONTAL: Vector3 = Vector3 {x: VIEWPORT_WIDTH, y: 0.0, z: 0.0};
const VERTICAL: Vector3 = Vector3 {x: 0.0, y: VIEWPORT_HEIGHT, z: 0.0};


fn write_header(file: &mut BufWriter<File>) {
    writeln!(file, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
}

fn write_color(file: &mut BufWriter<File>, color: &Color) {
    writeln!(file, "{} {} {}", (color.x * 255.999) as i32, (color.y * 255.999) as i32, (color.z * 255.999) as i32);
}

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - *center;

    let a = ray.direction.length_squared();
    let half_b = dot_product(&ray.direction, &oc);
    let c = oc.length_squared() - radius * radius;

    let discriminant  = (half_b * half_b) - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(ray: &Ray) -> Color {
    let sphere_center = Point3::new(0.0, 0.0, -1.0);
    let t = hit_sphere(&sphere_center, 0.5, ray);
    if t > 0.0 {
        let n = unit_vector(&(ray.at(t) - sphere_center));
        return Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5
    }

    // Get the normalized direction vector 
    let unit_direction = unit_vector(&ray.direction);

    let t = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let path = Path::new("output1.ppm");
    let file = match File::create(&path) {
        Err(why) => panic!("Couldn't create file {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut buf = BufWriter::new(file);
    
    write_header(&mut buf);

    let lower_left_corner = CAMERA_ORIGIN - (HORIZONTAL / 2.0) - (VERTICAL / 2.0) - Vector3::new(0.0, 0.0, FOCAL_LENGTH);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / (IMAGE_WIDTH - 1) as f64;
            let v = (j as f64) / (IMAGE_HEIGHT - 1) as f64;

            let ray: Ray = Ray::new(CAMERA_ORIGIN, lower_left_corner + HORIZONTAL * u + VERTICAL * v - CAMERA_ORIGIN);
            let pixel_color: Color = ray_color(&ray);
            write_color(&mut buf, &pixel_color);
        }
    }

    buf.flush().unwrap();
}
