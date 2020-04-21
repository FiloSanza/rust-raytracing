use super::ray::{Ray, ScatteredRay};
use super::utils::color::Color;
use super::hittable::HitRecord;
use super::material::Material;
use super::utils::vec3::Vec3;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> Option<ScatteredRay> {
        let scatter_direction = Vec3::random_in_unit_sphere();
        Some(ScatteredRay::new(
            Ray::new(record.point, scatter_direction),
            self.albedo
        ))
    }
}