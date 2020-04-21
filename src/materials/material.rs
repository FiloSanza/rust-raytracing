use super::ray::{Ray, ScatteredRay};
use super::hittable::HitRecord;

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<ScatteredRay>;
}