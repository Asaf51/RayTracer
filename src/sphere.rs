use std::rc::Rc;
use crate::hittable::{Hit, HitRecord};
use crate::material::Material;
use super::{Point3, Ray, dot_product};


#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Rc<dyn Material>
}

impl Sphere {
    pub fn new (center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {center, radius, material}
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;

        let a = ray.direction.length_squared();
        let half_b = dot_product(&ray.direction, &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant  = (half_b * half_b) - a * c;
        if discriminant <= 0.0 {
            return None
        }


        if discriminant > 0.0 {
            let res = (-half_b - discriminant.sqrt()) / a;

            if res < max_t && res > min_t {
                let point = ray.at(res);

                let mut hit_record = HitRecord {
                    t: res,
                    point: point,
                    normal: (point - self.center) / self.radius,
                    front_face: false,
                    material: self.material.clone()
                };

                let outward_normal = (hit_record.point - self.center) / self.radius;
                hit_record.set_face_normal(ray, outward_normal);
                return Some(hit_record)
            }

            let res = (-half_b + discriminant.sqrt()) / a;
            if res < max_t && res > min_t {
                let point = ray.at(res);

                let mut hit_record = HitRecord {
                    t: res,
                    point: point,
                    normal: (point - self.center) / self.radius,
                    front_face: false,
                    material: self.material.clone()
                };

                let outward_normal = (hit_record.point - self.center) / self.radius;
                hit_record.set_face_normal(ray, outward_normal);
                return Some(hit_record)
            }
        }

        None
    }
}