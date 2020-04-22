use super::utils::ray::{Ray, ScatteredRay};
use super::textures::Texture;
use super::hittable::HitRecord;
use super::material::Material;
use super::utils::vec3::Vec3;

use std::rc::Rc;

pub struct Isotropic {
    albedo: Rc<dyn Texture>
}

impl Isotropic {
    pub fn new(albedo: Rc<dyn Texture>) -> Self {
        Self {
            albedo
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<ScatteredRay> {
        Some(ScatteredRay::new(
            Ray::new(record.point, Vec3::random_in_unit_sphere(), ray.time),
            self.albedo.color(record.u, record.v, &record.point)
        ))
    }
}