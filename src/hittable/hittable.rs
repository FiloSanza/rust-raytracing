use super::material::material::Material;
use super::bounding::BoundingBox;
use super::{min_f64, max_f64};
use super::vec3::Vec3;
use super::ray::Ray;

use std::rc::Rc;
use std::f64;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub time: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>
}

impl HitRecord {
    pub fn new(ray: &Ray, point: Vec3, mut normal: Vec3, time: f64, u: f64, v: f64, material: Rc<dyn Material>) -> Self {
        let front_face = Vec3::dot_product(ray.direction, normal) < 0.0;
        normal = if front_face { normal } else { -normal };
        Self {
            point,
            normal,
            time,
            u,
            v,
            front_face,
            material
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<BoundingBox>;
}

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: vec![]
        }
    }

    pub fn new_from_vec(objects: Vec<Rc<dyn Hittable>>) -> Self {
        Self {
            objects
        }
    }

    pub fn push(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, tmin: f64, mut tmax: f64) -> Option<HitRecord> {
        let mut result = None;

        for object in self.objects.iter() {
            if let Some(record) = object.hit(ray, tmin, tmax) {
                tmax = record.time;
                result = Some(record);
            }
        }

        result
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<BoundingBox> {
        if self.objects.is_empty() {
            return None;
        }

        let mut first = false;
        let mut result = BoundingBox::default();

        for object in self.objects.iter() {
            if let Some(obj_box) = object.bounding_box(t0, t1) {
                result = if first { obj_box } else { BoundingBox::surrounding_box(&obj_box, &result) };
                first = false;
            }
            else {
                return None;
            }
        }

        Some(result)
    }
}

pub struct FlipFace {
    object: Rc<dyn Hittable>,
}

impl FlipFace {
    pub fn new(object: Rc<dyn Hittable>) -> Self {
        Self {
            object
        }
    }
}

impl Hittable for FlipFace {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        if let Some(mut record) = self.object.hit(ray, tmin, tmax){
            record.front_face = !record.front_face;
            Some(record)
        }
        else{
            None
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<BoundingBox> {
        self.object.bounding_box(t0, t1)
    }
}

pub struct Translate {
    offset: Vec3,
    object: Rc<dyn Hittable>,
}

impl Translate {
    pub fn new(object: Rc<dyn Hittable>, offset: Vec3) -> Self {
        Self {
            offset,
            object
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let translated_ray = Ray {
            origin: ray.origin - self.offset,
            ..*ray
        };

        if let Some(record) = self.object.hit(&translated_ray, tmin, tmax) {
            Some(HitRecord::new(
                &translated_ray,
                record.point + self.offset,
                record.normal,
                record.time,
                record.u,
                record.v,
                record.material
            ))            
        }
        else{
            None
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<BoundingBox> {
        if let Some(obj_box) = self.object.bounding_box(t0, t1) {
            Some(BoundingBox::new(
                obj_box.min + self.offset,
                obj_box.max + self.offset
            ))
        }
        else {
            None
        }
    }
}

pub struct RotateY {
    sin: f64,
    cos: f64,
    object: Rc<dyn Hittable>,
    obj_box: BoundingBox
}

impl RotateY {
    pub fn new(object: Rc<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin = radians.sin();
        let cos = radians.cos();
        let obj_box = object.bounding_box(0.0, 1.0).unwrap();

        let mut min = Vec3::new(f64::MAX, f64::MAX, f64::MAX);
        let mut max = Vec3::new(f64::MIN, f64::MIN, f64::MIN);
    
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * obj_box.max.x + (1.0 - i as f64) * obj_box.min.x;
                    let y = j as f64 * obj_box.max.y + (1.0 - j as f64) * obj_box.min.y;
                    let z = k as f64 * obj_box.max.z + (1.0 - k as f64) * obj_box.min.z;
                
                    let newx = cos * x + sin * z;
                    let newz = -sin * x + cos * z;
                
                    let tmp = Vec3::new(newx, y, newz);

                    for idx in 0..3 {
                        min[idx] = min_f64(min[idx], tmp[idx]);
                        max[idx] = max_f64(max[idx], tmp[idx]);
                    }
                }
            }
        }
        
        Self {
            sin,
            cos,
            object,
            obj_box: BoundingBox::new(min, max),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let mut origin = ray.origin;
        let mut direction = ray.direction;
    
        origin.x = self.cos * origin.x - self.sin * origin.z;
        origin.z = self.sin * origin.x + self.cos * origin.z;
    
        direction.x = self.cos * direction.x - self.sin * direction.z;
        direction.z = self.sin * direction.x + self.cos * direction.z;

        let rotated_ray = Ray::new(origin, direction, ray.time);

        if let Some(record) = self.object.hit(&rotated_ray, tmin, tmax) {
            let mut point = record.point;
            let mut normal = record.normal;

            point.x = self.cos * point.x + self.sin * point.z;
            point.z = self.cos * point.z - self.sin * point.x;
        
            normal.x = self.cos * normal.x + self.sin * normal.z;
            normal.z = self.cos * normal.z - self.sin * normal.x;

            Some(HitRecord::new(
                &rotated_ray,
                point, 
                normal,
                record.time,
                record.u,
                record.v,
                record.material.clone()
            ))
        }
        else{
            None
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<BoundingBox> {
        Some(self.obj_box)
    }
}