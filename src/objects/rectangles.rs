use super::hittable::{HitRecord, Hittable};
use super::material::material::Material;
use super::bounding::BoundingBox;
use super::vec3::Vec3;
use super::ray::Ray;

use std::sync::Arc;

pub struct XYRectangle {
    x0: f64, 
    x1: f64,
    y0: f64,
    y1: f64,
    z: f64,
    material: Arc<dyn Material>
}

impl XYRectangle {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, z: f64, material: Arc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            z,
            material
        }
    }
}

impl Hittable for XYRectangle {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let time = (self.z - ray.origin.z) / ray.direction.z;
    
        if time < tmin || time > tmax {
            return None;
        }

        let x = ray.at(time).x;
        let y = ray.at(time).y;

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        Some(HitRecord::new(
            ray,
            ray.at(time),
            Vec3::new(0.0, 0.0, 1.0),
            time,
            (x - self.x0) / (self.x1 - self.x0),
            (y - self.y0) / (self.y1 - self.y0),
            self.material.clone()
        ))
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<BoundingBox> {
        Some(BoundingBox::new(
            Vec3::new(self.x0, self.y0, self.z - 0.0001),
            Vec3::new(self.x1, self.y1, self.z + 0.0001)
        ))
    }
}

pub struct XZRectangle {
    x0: f64, 
    x1: f64,
    z0: f64,
    z1: f64,
    y: f64,
    material: Arc<dyn Material>
}

impl XZRectangle {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, y: f64, material: Arc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            y,
            material
        }
    }
}

impl Hittable for XZRectangle {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let time = (self.y - ray.origin.y) / ray.direction.y;
    
        if time < tmin || time > tmax {
            return None;
        }

        let x = ray.at(time).x;
        let z = ray.at(time).z;

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        Some(HitRecord::new(
            ray,
            ray.at(time),
            Vec3::new(0.0, 1.0, 0.0),
            time,
            (x - self.x0) / (self.x1 - self.x0),
            (z - self.z0) / (self.z1 - self.z0),
            self.material.clone()
        ))
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<BoundingBox> {
        Some(BoundingBox::new(
            Vec3::new(self.x0, self.y - 0.0001, self.z0),
            Vec3::new(self.x1, self.y + 0.0001, self.z1)
        ))
    }
}


pub struct YZRectangle {
    y0: f64, 
    y1: f64,
    z0: f64,
    z1: f64,
    x: f64,
    material: Arc<dyn Material>
}

impl YZRectangle {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, x: f64, material: Arc<dyn Material>) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            x,
            material
        }
    }
}

impl Hittable for YZRectangle {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let time = (self.x - ray.origin.x) / ray.direction.x;
    
        if time < tmin || time > tmax {
            return None;
        }

        let y = ray.at(time).y;
        let z = ray.at(time).z;

        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        Some(HitRecord::new(
            ray,
            ray.at(time),
            Vec3::new(1.0, 0.0, 0.0),
            time,
            (y - self.y0) / (self.y1 - self.y0),
            (z - self.z0) / (self.z1 - self.z0),
            self.material.clone()
        ))
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<BoundingBox> {
        Some(BoundingBox::new(
            Vec3::new(self.x - 0.0001, self.y0, self.z0),
            Vec3::new(self.x + 0.0001, self.y1, self.z1),
        ))
    }
}

