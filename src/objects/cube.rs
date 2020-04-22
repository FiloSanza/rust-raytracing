use super::hittable::{HitRecord, Hittable, HittableList, FlipFace};
use super::material::material::Material;
use super::bounding::BoundingBox;
use super::rectangles::*;
use super::vec3::Vec3;
use super::ray::Ray;

use std::rc::Rc;

pub struct Cube {
    top_right: Vec3,
    bottom_left: Vec3,
    sides: HittableList,
}

impl Cube {
    pub fn from_vertices(bottom_left: Vec3, top_right: Vec3, material: Rc<dyn Material>) -> Self {
        let mut sides = HittableList::new();
        
        //front
        sides.push(Rc::new(XYRectangle::new(
            bottom_left.x, top_right.x, bottom_left.y, top_right.y, top_right.z,
            material.clone()
        )));

        //back
        sides.push(Rc::new(FlipFace::new(Rc::new(XYRectangle::new(
            bottom_left.x, top_right.x, bottom_left.y, top_right.y, bottom_left.z,
            material.clone()
        )))));

        //top
        sides.push(Rc::new(XZRectangle::new(
            bottom_left.x, top_right.x, bottom_left.z, top_right.z, top_right.y,
            material.clone()
        )));

        //bottom
        sides.push(Rc::new(FlipFace::new(Rc::new(XZRectangle::new(
            bottom_left.x, top_right.x, bottom_left.z, top_right.z, bottom_left.y,
            material.clone()
        )))));

        //right
        sides.push(Rc::new(YZRectangle::new(
            bottom_left.y, top_right.y, bottom_left.z, top_right.z, top_right.x,
            material.clone()
        )));

        //left
        sides.push(Rc::new(FlipFace::new(Rc::new(YZRectangle::new(
            bottom_left.y, top_right.y, bottom_left.z, top_right.z, bottom_left.x,
            material.clone()
        )))));
    
        Self {
            sides,
            bottom_left,
            top_right,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        self.sides.hit(ray, tmin, tmax)
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<BoundingBox> {
        Some(BoundingBox::new(
            self.bottom_left,
            self.top_right
        ))
    }
}