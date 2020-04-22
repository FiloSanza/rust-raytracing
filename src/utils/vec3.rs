use std::ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut};
use std::f64::consts::PI;
use std::f64;

use super::min_f64;

use rand::distributions::{Uniform, Distribution};
use rand::Rng;

#[derive(Debug, Copy, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z
        }
    }

    pub fn random_range(from: f64, to: f64) -> Self {
        let mut rng = rand::thread_rng();
        let range = Uniform::from(from..to);

        Self {
            x: range.sample(&mut rng),
            y: range.sample(&mut rng),
            z: range.sample(&mut rng)
        }
    }

    pub fn random_unit() -> Self {
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(0.0, 2.0 * PI);
        let z = rng.gen_range(-1.0f64, 1.0f64);
        let r = (1.0 - z * z).sqrt();

        Self {
            x: r * a.cos(),
            y: r * a.sin(),
            z: z,
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let v = Vec3::random_range(-1.0, 1.0);

            if v.squared_length() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let v = Vec3::new(
                rng.gen_range(-1.0, 1.0), 
                rng.gen_range(-1.0, 1.0), 
                0.0
            );

            if v.squared_length() < 1.0 {
                return v;
            }
        }
    }

    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn dot_product(left: Self, right: Self) -> f64 {
        left.x * right.x + left.y * right.y + left.z * right.z
    }

    pub fn cross_product(left: Self, right: Self) -> Self {
        Self {
            x: left.y * right.z - left.z * right.y,
            y: left.z * right.x - left.x * right.z,
            z: left.x * right.y - left.y * right.x,
        }
    }

    pub fn reflect(v: Self, normal: Self) -> Self {
        v - normal * 2.0 * Self::dot_product(v, normal)
    }

    pub fn refract(v: Self, normal: Self, coeff: f64) -> Self {
        let cos = min_f64(1.0, Self::dot_product(-v, normal));
        let r_parallel = (v + normal * cos) * coeff;
        let r_perpendicular = -normal * (1.0 - r_parallel.squared_length()).sqrt();
        
        r_parallel + r_perpendicular
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bound")
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bound")
        }
    }
}


impl Clone for Vec3 {
    fn clone(&self) -> Self {
        *self
    }
}

impl Add for Vec3 {
type Output = Self;

    fn add(self, other: Self) -> Self {
        Self{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self{
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Div for Vec3 {
type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f64> for Vec3 {
type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Neg for Vec3 {
type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}