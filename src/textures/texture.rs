use super::utils::color::Color;
use super::utils::vec3::Vec3;

pub trait Texture {
    fn color(&self, u: f64, v: f64, point: &Vec3) -> Color;
}