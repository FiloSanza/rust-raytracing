use super::utils::ray::{Ray, ScatteredRay};
use super::textures::Texture;
use super::hittable::HitRecord;
use super::material::Material;
use super::utils::vec3::Vec3;

use std::rc::Rc;

pub struct Lambertian {
    albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Rc<dyn Texture>) -> Self {
        Self {
            albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<ScatteredRay> {
        let scatter_direction = record.normal + Vec3::random_unit();
        Some(ScatteredRay::new(
            Ray::new(
                record.point, 
                scatter_direction,
                ray.time
            ),
            self.albedo.color(record.u, record.v, &record.point)
        ))
    }
}