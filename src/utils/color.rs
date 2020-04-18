use std::ops::{Mul, Div, Add, Sub};

use rand::distributions::{Distribution, Uniform}; 
use num::clamp;

#[derive(Debug, Default, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            r,
            g,
            b,
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let range = Uniform::from(0.0..1.0);

        Self {
            r: range.sample(&mut rng),
            g: range.sample(&mut rng),
            b: range.sample(&mut rng)
        }
    }

    pub fn to_rgb(&self) -> String {
        let r = (255.0 * self.r).sqrt() as i32;
        let g = (255.0 * self.g).sqrt() as i32;
        let b = (255.0 * self.b).sqrt() as i32;
        format!("{} {} {}", r, g, b)
    }

    pub fn to_rgb_with_samples(&self, samples: i32) -> String {
        let scale = 1.0 / samples as f64;
        let r = (255.0 * clamp(self.r * scale, 0.0, 1.0).sqrt()) as i32;
        let g = (255.0 * clamp(self.g * scale, 0.0, 1.0).sqrt()) as i32;
        let b = (255.0 * clamp(self.b * scale, 0.0, 1.0).sqrt()) as i32;

        

        format!("{} {} {}", clamp(r, 0, 255), clamp(g, 0, 255), clamp(b, 0, 255))
    }
}

impl Clone for Color {
    fn clone(&self) -> Self {
        *self
    }
}

impl Add for Color {
type Output = Self;

    fn add(self, other: Self) -> Self {
        Self{
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Sub for Color {
type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self{
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl Mul for Color {
type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl Div for Color {
type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            r: self.r / other.r,
            g: self.g / other.g,
            b: self.b / other.b,
        }
    }
}

impl Mul<f64> for Color {
type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl Div<f64> for Color {
type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            r: self.r / other,
            g: self.g / other,
            b: self.b / other,
        }
    }
}