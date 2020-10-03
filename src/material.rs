use crate::hittable::HitRecord;
use crate::vec3::{Color, Vector3, dot_product, unit_vector};
use crate::ray::Ray;

pub struct MaterialInfo {
    pub attenuation: Color,
    pub scattered: Ray
}

pub trait Material {
    fn scatter (&self, ray: &Ray, hit_record: &HitRecord) -> Option<MaterialInfo>;
}

pub struct Lambertian {
    pub albedo: Color
}

pub struct Metal {
    pub albedo: Color
}

fn reflect(vec: &Vector3, normal: &Vector3) -> Vector3 {
    *vec -  *normal * 2.0 * dot_product(vec, normal)
}

impl Material for Metal {
    fn scatter (&self, ray: &Ray, hit_record: &HitRecord) -> Option<MaterialInfo> {
        let reflected = reflect(&unit_vector(&ray.direction), &hit_record.normal);
        let scattered = Ray::new(hit_record.point, reflected);

        if dot_product(&scattered.direction, &hit_record.normal) > 0.0 {
            Some(MaterialInfo {
                scattered: Ray::new(hit_record.point, reflected),
                attenuation: self.albedo
            })
        } else {
            None
        }
        
    }
}

impl Material for Lambertian {
    fn scatter (&self, _ray: &Ray, hit_record: &HitRecord) -> Option<MaterialInfo> {
        let scatter_direction = hit_record.normal + Vector3::new_random_unit();

        Some(MaterialInfo {
            attenuation: self.albedo,
            scattered: Ray::new(hit_record.point, scatter_direction)
        })
    }
}
