use super::hittable::{HitRecord, Hittable};
use super::material::material::Material;
use super::bounding::BoundingBox;
use super::vec3::Vec3;
use super::ray::Ray;

use std::f64::consts::PI;
use std::rc::Rc;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Rc<dyn Material>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material
        }
    }

    fn get_hit_record(&self, ray: &Ray, time: f64) -> HitRecord {
        let point = ray.at(time);
        let (u, v) = self.get_uv((point - self.center) / self.radius);
        HitRecord::new(
            ray,
            point,
            (point - self.center) / self.radius,
            time,
            u,
            v,
            self.material.clone()
        )
    }

    fn get_uv(&self, point: Vec3) -> (f64, f64) {
        let phi = point.z.atan2(point.x);
        let theta = point.y.asin();
        
        (1.0 - (phi + PI) / 2.0 * PI, (theta + PI / 2.0) / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.squared_length();
        let b = Vec3::dot_product(oc, ray.direction);
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let time = (-b - root) / a;
            
            if time < tmax && time > tmin {
                return Some(self.get_hit_record(ray, time));
            }

            let time = (-b + root) / a;
            if time < tmax && time > tmin {
                return Some(self.get_hit_record(ray, time));
            }
        }
    
        None
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<BoundingBox> {
        Some(BoundingBox::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius)
        ))
    }
}
