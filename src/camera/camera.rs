use super::utils::vec3::Vec3;
use super::ray::Ray;

#[derive(Debug, Copy)]
pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Clone for Camera {
    fn clone(&self) -> Self {
        *self
    }
}

impl Camera {
    pub fn new(origin: Vec3, look_at: Vec3, up: Vec3, fov: f64, aspect: f64, aperture: f64, focus_distance: f64) -> Self {
        let theta = fov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (origin - look_at).unit_vector();
        let u = Vec3::cross_product(up, w).unit_vector();
        let v = Vec3::cross_product(w, u);
        
        Self {
            origin,
            lower_left: origin - (u * half_width + v * half_height + w) * focus_distance,
            horizontal: u * half_width * 2.0 * focus_distance,
            vertical: v * half_height * 2.0 * focus_distance,
            lens_radius: aperture / 2.0,
            u,
            w,
            v,
        }
    }

    pub fn get_ray(&self, x: f64, y: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(self.origin + offset, self.lower_left + self.horizontal * x + self.vertical * y - self.origin - offset)
    }
}