use super::material::{isotropic::Isotropic, material::Material};
use super::hittable::{HitRecord, Hittable};
use super::bounding::BoundingBox;
use super::{min_f64, max_f64};
use super::textures::Texture;
use super::vec3::Vec3;
use super::ray::Ray;

use std::sync::Arc;
use std::f64;

use rand::Rng;

pub struct ConstantMedium {
    object: Arc<dyn Hittable>,
    material: Arc<dyn Material>,
    density: f64,
}

impl ConstantMedium {
    pub fn new(object: Arc<dyn Hittable>, texture: Arc<dyn Texture>, density: f64) -> Self {
        Self {
            object,
            density: -1.0/density,
            material: Arc::new(Isotropic::new(texture)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let mut record1 = if let Some(record) = self.object.hit(ray, f64::MIN, f64::MAX) {
            record
        }
        else{
            return None;
        };

        
        let mut record2 = if let Some(record) = self.object.hit(ray, record1.time + 0.0001, f64::MAX) {
            record
        }
        else{
            return None;
        };
        
        record1.time = max_f64(tmin, record1.time);
        record2.time = min_f64(tmax, record2.time);
        
        if record1.time >= record2.time {
            return None;
        }

        record1.time = max_f64(0.0, record1.time);

        let mut rng = rand::thread_rng();
        let ray_length = ray.direction.length();
        let distance_inside = (record2.time - record1.time) * ray_length;
        let hit_distance = self.density * (rng.gen_range(0.0, 1.0) as f64).ln();

        if hit_distance > distance_inside {
            return None;
        }

        let time = record1.time + hit_distance / ray_length;
        Some(HitRecord::new(
            &ray,
            ray.at(time),
            Vec3::new(1.0, 0.0, 0.0),  //doesn't matter
            time,
            0.0,
            0.0,
            self.material.clone()
        ))
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<BoundingBox> {
        self.object.bounding_box(t0, t1)
    }
}