use super::hittable::{HitRecord, Hittable};
use super::material::material::Material;
use super::bounding::BoundingBox;
use super::vec3::Vec3;
use super::ray::Ray;

use std::f64::consts::PI;
use std::sync::Arc;

pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    time0: f64,
    time1: f64,
    radius: f64,
    material: Arc<dyn Material>
}

impl MovingSphere {
    pub fn new(center0: Vec3, center1: Vec3, time0: f64, time1: f64, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material
        }
    }

    fn find_center(&self, time: f64) -> Vec3 {
        self.center0 + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
    }

    fn get_hit_record(&self, ray: &Ray, time: f64) -> HitRecord {
        let point = ray.at(time);
        let (u, v) = self.get_uv((point - self.find_center(time)) / self.radius);
        HitRecord::new(
            ray,
            point,
            (point - self.find_center(time)) / self.radius,
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

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.find_center(ray.time);
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

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<BoundingBox> {
        let min = BoundingBox::new(
            self.find_center(t0) - Vec3::new(self.radius, self.radius, self.radius),
            self.find_center(t0) + Vec3::new(self.radius, self.radius, self.radius)
        );

        let max = BoundingBox::new(
            self.find_center(t1) - Vec3::new(self.radius, self.radius, self.radius),
            self.find_center(t1) + Vec3::new(self.radius, self.radius, self.radius)
        );

        Some(BoundingBox::surrounding_box(
            &min,
            &max
        ))
    }
}