use super::vec3::Vec3;
use super::ray::Ray;
use super::hittable::{Hittable, HitRecord};
use super::material::Material;

use std::sync::{Arc};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.squared_length();
        let b = Vec3::dot_product(oc, ray.direction);
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = b*b - a*c;

        if discriminant > 0.0 {
            let time = (-b - discriminant.sqrt()) / a;
            if time > t_min && time < t_max {
                let point = ray.at(time);
                let normal = (point - self.center) / self.radius;
                return Some(HitRecord::new(
                    &ray, 
                    point, 
                    normal, 
                    time, 
                    self.material.clone()
                ));
            }
            let time = (-b + discriminant.sqrt()) / a;
            if time > t_min && time < t_max {
                let point = ray.at(time);
                let normal = (point - self.center) / self.radius;
                return Some(HitRecord::new(
                    &ray, 
                    point, 
                    normal, 
                    time, 
                    self.material.clone()
                ));
            }
        }

        None
    }
}