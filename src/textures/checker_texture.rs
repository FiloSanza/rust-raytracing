use super::utils::color::Color;
use super::utils::vec3::Vec3;
use super::texture::Texture;

use std::rc::Rc;

pub struct CheckerTexture {
    odd: Rc<dyn Texture>,
    even: Rc<dyn Texture>
}

impl CheckerTexture {
    pub fn new(odd: Rc<dyn Texture>, even: Rc<dyn Texture>) -> Self {
        Self {
            odd,
            even
        }
    }
}

impl Texture for CheckerTexture {
    fn color(&self, u: f64, v: f64, point: &Vec3) -> Color {
        let sin = (10.0 * point.x).sin() * (10.0 * point.y).sin() * (10.0 * point.z).sin();

        if sin < 0.0 {
            self.odd.color(u, v, point)
        }
        else {
            self.even.color(u, v, point)
        }
    }
}