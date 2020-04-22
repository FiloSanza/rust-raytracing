use super::utils::ray::{Ray, ScatteredRay};
use super::hittable::HitRecord;
use super::utils::color::Color;
use super::utils::vec3::Vec3;

pub trait Material: Send + Sync {
    fn scatter(&self, _ray: &Ray, _record: &HitRecord) -> Option<ScatteredRay> {
        None
    }

    fn emit(&self, _u: f64, _x: f64, _point: &Vec3) -> Color {
        Color::default()
    }
}