use super::utils::ray::{Ray, ScatteredRay};
use super::hittable::HitRecord;
use super::utils::color::Color;
use super::material::Material;
use super::utils::vec3::Vec3;
use super::utils::min_f64;

use rand::Rng;

pub struct Dielectric {
    refraction: f64
}

impl Dielectric {
    pub fn new(refraction: f64) -> Self {
        Self {
            refraction
        }
    }

    fn schlick(cos: f64, index: f64) -> f64 {
        let mut r0 = (1.0 - index) / (1.0 + index);
        r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<ScatteredRay> {
        let index = if record.front_face { 1.0 / self.refraction } else { self.refraction };
        let unit_direction = ray.direction.unit_vector();

        let cos = min_f64(Vec3::dot_product(-unit_direction, record.normal), 1.0);
        let sin = (1.0 - cos * cos).sqrt();

        let reflect_prob = Self::schlick(cos, index);
        let mut rng = rand::thread_rng();

        let result = if reflect_prob > rng.gen_range(0.0, 1.0) || index * sin > 1.0 {
            Vec3::reflect(unit_direction, record.normal)
        }
        else{
            Vec3::refract(unit_direction, record.normal, index)
        };

        Some(ScatteredRay::new(
            Ray::new(
                record.point,
                result,
                ray.time
            ),
            Color::new(1.0, 1.0, 1.0)
        ))
    }
}