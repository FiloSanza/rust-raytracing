pub mod vec3;
pub mod color;

pub fn min_f64(left: f64, right: f64) -> f64 {
    if left <= right {
        left
    }
    else {
        right
    }
}