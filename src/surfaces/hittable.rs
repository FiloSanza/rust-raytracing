use std::sync::{Arc};

use super::material::Material;
use super::vec3::Vec3;
use super::ray::Ray;

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub time: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(ray: &Ray, point: Vec3, mut normal: Vec3, time: f64, material: Arc<dyn Material>) -> Self {
        let front_face = Vec3::dot_product(ray.direction, normal) < 0.0;
        normal = if front_face { normal } else { -normal };

        Self {
            point,
            normal,
            time,
            front_face,
            material
        }
    }
}

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new()
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object)
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, mut t_max: f64) -> Option<HitRecord> {
        let mut result = None;

        for object in self.objects.iter() {
            if let Some(record) = object.hit(&ray, t_min, t_max) {
                t_max = record.time;
                result = Some(record);
            }
        }

        result
    }
}

