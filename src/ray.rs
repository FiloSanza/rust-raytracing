use super::utils::vec3::Vec3;
use super::utils::color::Color;

#[derive(Debug, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction
        }
    }

    pub fn at(&self, time: f64) -> Vec3 {
        self.origin + self.direction * time
    }
}

impl Clone for Ray {
    fn clone(&self) -> Self {
        *self
    }
}

pub struct ScatteredRay {
    pub ray: Ray,
    pub attenuation: Color,
}

impl ScatteredRay {
    pub fn new(ray: Ray, attenuation: Color) -> Self {
        Self {
            ray,
            attenuation,
        }
    }
}
