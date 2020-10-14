use super::{Point3, Vector3, Ray, unit_vector, cross};

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vector3,
    pub vertical: Vector3
}

impl Camera {
    pub fn new(lookfrom: Point3,
               lookat: Point3, 
               vup: Vector3, 
               vfov: f64, 
               aspect_ratio: f64) -> Self {
        let viewport_height = 2.0 * (vfov.to_radians() / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(&(lookfrom - lookat));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);

        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = lookfrom - (horizontal / 2.0) - (vertical / 2.0) - w;
        Self {
            origin: lookfrom,
            lower_left_corner,
            horizontal,
            vertical
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin)
    }
}
