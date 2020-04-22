use super::{Ray, Vec3};

use rand::Rng;

pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64
}

impl Camera {
    pub fn new(origin: Vec3, look_at: Vec3, up: Vec3, fov: f64, aspect: f64, aperture: f64, focus_dist: f64, time0: f64, time1: f64) -> Self {
        let theta = fov.to_radians();
        let height = (theta / 2.0).tan();
        let width = aspect * height;
        
        let w = (origin - look_at).unit_vector();
        let u = Vec3::cross_product(up, w).unit_vector();
        let v = Vec3::cross_product(w, u);

        let lower_left = origin - (u * width + v * height + w) * focus_dist;
        let horizontal = u * 2.0 * focus_dist * width;
        let vertical = v * 2.0 * focus_dist * height;

        Self {
            origin,
            lower_left,
            horizontal,
            vertical,
            v,
            u,
            w,
            lens_radius: aperture / 2.0,
            time0,
            time1
        }
    }

    pub fn get_ray(&self, x: f64, y: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        let mut rng = rand::thread_rng();

        Ray::new(
            self.origin + offset,
            self.lower_left + self.horizontal * x + self.vertical * y - self.origin - offset,
            rng.gen_range(self.time0, self.time1)
        )
    }
}