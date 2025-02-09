use crate::hittable::HitRecord;
use crate::vec3::{Color, Vector3, dot_product, unit_vector, random_in_unit_sphere};
use crate::ray::Ray;
use rand::Rng;

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
    pub albedo: Color,
    pub fuzz: f64
}

pub struct Dielectric {
    pub ref_idx: f64
}

pub fn reflectance (cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

impl Metal {
    pub fn new (albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 }  
        }
    }
}

impl Dielectric {
    pub fn new (ref_idx: f64) -> Self {
        Self { ref_idx }
    }
}

fn refract(uv: &Vector3, normal: &Vector3, etai_over_etat: f64) -> Vector3 {
    let cos_theta = dot_product(&(*uv * -1.0), normal);
    let out_perp = (*uv + *normal * cos_theta) * etai_over_etat;
    let out_parallel = *normal * -1.0 * (1.0 - out_perp.length_squared()).abs().sqrt();

    out_perp + out_parallel
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<MaterialInfo> {
        let etai_over_etat = if hit_record.front_face { 1.0 / self.ref_idx } else { self.ref_idx };
        let unit_direction = unit_vector(&ray.direction);

        let mut cos_theta: f64 = dot_product(&(unit_direction * -1.0), &hit_record.normal);
        if cos_theta > 1.0 {
            cos_theta = 1.0;
        }

        let mut rng = rand::thread_rng();

        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let direction = if etai_over_etat * sin_theta > 1.0 || reflectance(cos_theta, etai_over_etat) > rng.gen_range(0.0, 1.0) { 
            reflect(&unit_direction, &hit_record.normal) 
        } else { 
            refract(&unit_direction, &hit_record.normal, etai_over_etat) 
        }; 
                
        Some(MaterialInfo {
            attenuation: Color::new(1.0, 1.0, 1.0),
            scattered: Ray::new(hit_record.point, direction)
        })
    }
}

fn reflect(vec: &Vector3, normal: &Vector3) -> Vector3 {
    *vec - *normal * 2.0 * dot_product(vec, normal)
}

impl Material for Metal {
    fn scatter (&self, ray: &Ray, hit_record: &HitRecord) -> Option<MaterialInfo> {
        let reflected = reflect(&unit_vector(&ray.direction), &hit_record.normal);
        let scattered = Ray::new(hit_record.point, reflected + (random_in_unit_sphere() * self.fuzz));

        if dot_product(&scattered.direction, &hit_record.normal) > 0.0 {
            Some(MaterialInfo {
                scattered,
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
