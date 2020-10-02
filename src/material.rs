use crate::hittable::HitRecord;
use crate::vec3::Color;
use crate::ray::Ray;

pub struct MaterialInfo {
    pub hit_record: HitRecord,
    pub color: Color,
    pub ray: Ray
}

pub trait Material {
    fn scatter (&self, ray: &Ray) -> Option<MaterialInfo>;
}

