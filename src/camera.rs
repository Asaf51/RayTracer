use super::{Point3, Vector3, Ray, unit_vector, cross, random_in_unit_disk};

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vector3,
    pub vertical: Vector3,
    pub lens_radius: f64,
    
    pub u: Vector3,
    pub v: Vector3,
    pub w: Vector3
}

impl Camera {
    pub fn new (lookfrom: Point3,
                lookat: Point3, 
                vup: Vector3, 
                vfov: f64, 
                aspect_ratio: f64,
                aperture: f64,
                focus_dist: f64) -> Self {
        let viewport_height = 2.0 * (vfov.to_radians() / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(&(lookfrom - lookat));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);

        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = lookfrom - (horizontal / 2.0) - (vertical / 2.0) - (w * focus_dist);
        Self {
            origin: lookfrom,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius: aperture / 2.0,
            u, v, w
        }
    }
    
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd: Vector3 = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(self.origin + offset, self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset)
    }
}
