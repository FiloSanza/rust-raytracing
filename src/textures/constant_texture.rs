use super::utils::color::Color;
use super::utils::vec3::Vec3;
use super::texture::Texture;

pub struct ConstantTexture {
    color: Color
}

impl ConstantTexture {
    pub fn new(color: Color) -> Self {
        Self {
            color
        }
    }
}

impl Texture for ConstantTexture {
    fn color(&self, _u: f64, _v: f64, _point: &Vec3) -> Color {
        self.color
    }
}