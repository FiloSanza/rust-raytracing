use super::texture::Texture;
use super::utils::color::Color;
use super::utils::vec3::Vec3;

use num::clamp;

use image::{DynamicImage, GenericImageView, Pixel};

pub struct ImageTexture {
    image: DynamicImage,
    width: usize,
    height: usize,
}

impl ImageTexture {
    pub fn new(image: DynamicImage) -> Self {
        let (width, height) = image.dimensions();
        Self {
            image,
            width: width as usize,
            height: height as usize,
        }
    }
}

impl Texture for ImageTexture {
    fn color(&self, u: f64, v: f64, _point: &Vec3) -> Color {
        let mut i = (u * self.width as f64) as usize;
        let mut j = ((1.0 - v) * self.height as f64) as usize;

        i = clamp(i, 0, self.width - 1);
        j = clamp(j, 0, self.height - 1);

        let rgb = self.image.get_pixel(i as u32, j as u32).to_rgb();

        Color::new(
            rgb[0] as f64 / 255.0,
            rgb[1] as f64 / 255.0,
            rgb[2] as f64 / 255.0,
        )
    }
}