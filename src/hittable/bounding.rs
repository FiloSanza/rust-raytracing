use super::hittable::{HitRecord, Hittable};
use super::{min_f64, max_f64};
use super::vec3::Vec3;
use super::ray::Ray;

use std::rc::Rc;
use std::cmp::Ordering;

#[derive(Default, Copy, Debug)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl BoundingBox {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self {
            min,
            max
        }
    }

    pub fn hit(&self, ray: &Ray, mut tmin: f64, mut tmax: f64) -> bool {
        for idx in 0..3 {
            let inverse_direction = 1.0 / ray.direction[idx];
            
            let mut t0 = (self.min[idx] - ray.origin[idx]) * inverse_direction;
            let mut t1 = (self.max[idx] - ray.origin[idx]) * inverse_direction;

            if inverse_direction < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            tmin = max_f64(tmin, t0);
            tmax = min_f64(tmax, t1);

            if tmax <= tmin {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(a: &Self, b: &Self) -> Self {
        let min = Vec3::new(
            min_f64(a.min.x, b.min.x),
            min_f64(a.min.y, b.min.y),
            min_f64(a.min.z, b.min.z)
        );

        let max = Vec3::new(
            max_f64(a.max.x, b.max.x),
            max_f64(a.max.y, b.max.y),
            max_f64(a.max.z, b.max.z)
        );

        Self {
            min,
            max
        }
    }
}

impl Clone for BoundingBox {
    fn clone(&self) -> Self {
        *self
    }
}

pub struct BvhNode {
    pub left: Rc<dyn Hittable>,
    pub right: Rc<dyn Hittable>,
    obj_box: BoundingBox
}

impl BvhNode {
    pub fn new(objects: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize, t0: f64, t1: f64) -> Self {
        let left;
        let right;
        let size = end - start;

        // FIXME: PICK RANDOM AXIS AND SORT ONLY THE SUBARRAY
        let axis = 0;

        match size {
            1 => {
                left = objects[start].clone();
                right = objects[start].clone();
            },
            2 => {
                if Self::box_compare(&objects[start], &objects[start+1], axis).unwrap() == Ordering::Less {
                    left = objects[start].clone();
                    right = objects[start+1].clone();
                }
                else{
                    left = objects[start+1].clone();
                    right = objects[start].clone(); 
                }
            },
            _ => {
                objects.sort_by(|a, b| {
                    Self::box_compare(&a, &b, axis).unwrap()
                });

                let mid = start + size / 2;
                left = Rc::new(BvhNode::new(objects, start, mid, t0, t1));
                right = Rc::new(BvhNode::new(objects, mid, end, t0, t1));
            }
        };

        let left_box = if let Some(obj_box) = left.bounding_box(t0, t1) {
            obj_box
        }
        else {
            panic!("No box for left object");
        };

        let right_box = if let Some(obj_box) = right.bounding_box(t0, t1) {
            obj_box
        }
        else {
            panic!("No box for right object");
        };

        Self {
            left,
            right,
            obj_box: BoundingBox::surrounding_box(&left_box, &right_box)
        }
    }   

    fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: usize) -> Option<Ordering> {
        let box_a = if let Some(obj_box) = a.bounding_box(0.0, 0.0) {
            obj_box
        }
        else {
            panic!("No box for parameter A");
        };

        let box_b = if let Some(obj_box) = b.bounding_box(0.0, 0.0) {
            obj_box
        }
        else {
            panic!("No box for parameter B");
        };

        box_a.min[axis].partial_cmp(&box_b.min[axis])
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        if !self.obj_box.hit(ray, tmin, tmax) {
            return None;
        }

        let hit_left = self.left.hit(ray, tmin, tmax);
        let hit_right = self.right.hit(ray, tmin, tmax);

        if hit_left.is_none() && hit_right.is_none() {
            None
        }
        else{
            if hit_left.is_none() {
                hit_right
            }
            else if hit_right.is_none() {
                hit_left
            }
            else{
                let left = hit_left.unwrap();
                let right = hit_right.unwrap();
                
                Some(if left.time < right.time { left } else { right })
            }
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<BoundingBox> {
        Some(self.obj_box)
    }
}