use super::material::Material;
use super::hittable::HitRecord;
use super::ray::{Ray, ScatteredRay};
use super::utils::vec3::Vec3;
use super::utils::color::Color;
use super::utils::min_f64;

use rand::Rng;

pub struct Dielectric {
    refraction: f64,
}

impl Dielectric {
    pub fn new(refraction: f64) -> Self {
        Self {
            refraction
        }
    }

    fn shlick(cos: f64, coeff: f64) -> f64 {
        let mut r0 = (1.0 - coeff) / (1.0 + coeff);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<ScatteredRay> {
        let coeff = if record.front_face { 1.0 / self.refraction } else { self.refraction };

        let unit_direction = ray.direction.unit_vector();
        let cos = min_f64(1.0, Vec3::dot_product(-unit_direction, record.normal));
        let sin = (1.0 - cos * cos).sqrt();

        let reflect_prob = Self::shlick(cos, coeff);
        let mut rng = rand::thread_rng();

        if coeff * sin > 1.0 || reflect_prob > rng.gen_range(0.0, 1.0) {
            let reflected = Vec3::reflect(unit_direction, record.normal);

            Some(ScatteredRay::new(
                Ray::new(record.point, reflected),
                Color::new(1.0, 1.0, 1.0)
            ))
        }
        else {
            let refracted = Vec3::refract(unit_direction, record.normal, coeff);

            Some(ScatteredRay::new(
                Ray::new(record.point, refracted),
                Color::new(1.0, 1.0, 1.0)
            ))
        }
    }
}
