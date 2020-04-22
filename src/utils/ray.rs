use super::vec3::Vec3;
use super::color::Color;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f64
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f64) -> Self {
        Self {
            origin,
            direction,
            time
        }
    }

    pub fn at(&self, time: f64) -> Vec3 {
        self.origin + self.direction * time
    }
}

pub struct ScatteredRay {
    pub ray: Ray,
    pub attenuation: Color
}

impl ScatteredRay {
    pub fn new(ray: Ray, attenuation: Color) -> Self {
        Self {
            ray,
            attenuation
        }
    }
}