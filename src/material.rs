use crate::hittable::HitData;
use crate::random_unit_vector;
use crate::ray::Ray;
use crate::vec3::{dot, reflect, Vec3};

#[derive(Copy, Clone)]
pub enum Material {
    Diffuse(Vec3),
    Metal(Vec3),
}

pub struct ScatterData {
    pub scattered: Ray,
    pub attenuation: Vec3,
}

pub fn scatter(r: &Ray, rec: &HitData) -> Option<ScatterData> {
    match rec.mat {
        Material::Diffuse(albedo) => {
            let scatter_direction = rec.normal + random_unit_vector();
            Some(ScatterData {
                scattered: Ray::new(rec.p, scatter_direction),
                attenuation: albedo,
            })
        }
        Material::Metal(albedo) => {
            let reflected = reflect(r.direction.unit(), rec.normal);
            let scattered = Ray::new(rec.p, reflected);
            if dot(scattered.direction, rec.normal) > 0.0 {
                return Some(ScatterData {
                    scattered,
                    attenuation: albedo,
                });
            }
            None
        }
    }
}
