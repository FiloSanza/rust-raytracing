pub mod color;
pub mod vec3;
pub mod ray;

pub fn min_f64(a: f64, b: f64) -> f64 {
    if a <= b { a } else { b }
}

pub fn max_f64(a: f64, b: f64) -> f64 {
    if a >= b { a } else { b }
}