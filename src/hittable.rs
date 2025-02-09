use std::vec::Vec;
use std::rc::Rc;
use std::boxed::Box;
use super::{Point3, Vector3, Ray, dot_product};
use crate::material::Material;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vector3,
    pub t: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3) {
        self.front_face = dot_product(&ray.direction, &outward_normal) < 0.0;
        self.normal = if self.front_face {outward_normal} else {outward_normal * -1.0};
    }
}

pub trait Hit {
    fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hit>>
}

impl HittableList {
    pub fn new() -> Self {
        Self {objects: Vec::new()}
    }
}

impl Hit for HittableList {
    fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<HitRecord> {
        let mut closest_so_far = max_t;
        let mut hit_record: Option<HitRecord> = None;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, min_t, closest_so_far) {
                closest_so_far = hit.t;
                hit_record = Some(hit);
            }
        }

        hit_record
    }
}
