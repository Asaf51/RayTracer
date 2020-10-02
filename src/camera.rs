use super::{Point3, Vector3, Ray};

// All those are consts for now, maybe configurable in the future :)
const ASPECT_RATIO: f64 = 16.0 / 10.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

// (0, 0, 0)
const CAMERA_ORIGIN: Point3 = Point3 {x: 0.0, y: 0.0, z: 0.0};
const HORIZONTAL: Vector3 = Vector3 {x: VIEWPORT_WIDTH, y: 0.0, z: 0.0};
const VERTICAL: Vector3 = Vector3 {x: 0.0, y: VIEWPORT_HEIGHT, z: 0.0};

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vector3,
    pub vertical: Vector3
}

impl Camera {
    pub fn new() -> Self {
        let lower_left_corner = CAMERA_ORIGIN - (HORIZONTAL / 2.0) - (VERTICAL / 2.0) - Vector3::new(0.0, 0.0, FOCAL_LENGTH);
        Self {
            origin: CAMERA_ORIGIN,
            lower_left_corner: lower_left_corner,
            horizontal: HORIZONTAL,
            vertical: VERTICAL
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin)
    }
}