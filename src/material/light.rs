use super::utils::color::Color;
use super::material::Material;
use super::textures::Texture;
use super::utils::vec3::Vec3;

use std::rc::Rc;

pub struct Light {
    emitted: Rc<dyn Texture>
}

impl Light {
    pub fn new(emitted: Rc<dyn Texture>) -> Self {
        Self {
            emitted
        }
    }
}

impl Material for Light {
    fn emit(&self, u: f64, v: f64, point: &Vec3) -> Color {
        self.emitted.color(u, v, point)
    }
}