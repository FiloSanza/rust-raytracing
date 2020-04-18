use super::ray::{Ray, ScatteredRay};
use super::hittable::HitRecord;
use super::utils::color::Color;
use super::material::Material;
use super::utils::vec3::Vec3;

pub struct Metal {
    albedo: Color,
    fuzziness: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzziness: f64) -> Self {
        Self {
            albedo,
            fuzziness
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<ScatteredRay> {
        let reflected = Vec3::reflect(ray.direction.unit_vector(), record.normal);
        let scattered = Ray::new(record.point, reflected + Vec3::random_in_unit_sphere() * self.fuzziness);
        if Vec3::dot_product(scattered.direction, record.normal) > 0.0 {
            Some(ScatteredRay::new(
                scattered,
                self.albedo
            ))
        }
        else {
            None
        }
    }
}