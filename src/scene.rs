use super::camera::camera::Camera;
use super::hittable::{Hittable};
use super::utils::color::Color;
use super::utils::ray::Ray;

use rand::distributions::{Distribution, Uniform};

use std::sync::Arc;
use std::cmp::min;
use std::thread;
use std::f64;

pub struct Scene {
    objects: Arc<dyn Hittable>,
    camera: Arc<Camera>,
    background: Arc<Color>,
}

impl Scene {
    pub fn new(objects: Arc<dyn Hittable>, camera: Camera, background: Color) -> Self {
        Self {
            objects,
            camera: Arc::new(camera),
            background: Arc::new(background)
        }
    }

    pub fn render(&self, width: usize, height: usize, samples: usize, depth: i32, threads: usize) -> Vec<Color> {
        let blocks = (height as f64 / threads as f64) as usize;
        let mut handles = Vec::new();

        for thrd in 0..threads {
            let camera = self.camera.clone();
            let objects = self.objects.clone();
            let background = self.background.clone();

            eprintln!("launching: {}", thrd);

            let handle = thread::spawn(move || {
                let mut rng = rand::thread_rng();
                let range = Uniform::from(0.0..1.0);
                let mut result = Vec::new();

                for row in (thrd * blocks)..min(width, blocks * (thrd + 1)) {
                    for col in 0..width{
                        let mut color = Color::default();
                        for _ in 0..samples {
                            let x = (col as f64 + range.sample(&mut rng)) / width as f64;
                            let y = ((height - row - 1) as f64 + range.sample(&mut rng)) / height as f64;
                        
                            let ray = camera.get_ray(x, y);

                            color = color + Self::get_color(&ray, &background, &objects, depth);
                        }
                        result.push(color);
                    }
                }
                
                eprintln!("done: {}", thrd);
                
                result
            });

            handles.push(handle);
        }

        let mut image = Vec::new();

        let mut idx = 0;
        for handle in handles {
            eprintln!("joining: {}", idx);
            image.append(&mut handle.join().unwrap());
            idx += 1;
        }

        image
    }

    fn get_color(ray: &Ray, background: &Arc<Color>, world: &Arc<dyn Hittable>, depth: i32) -> Color {
        if depth <= 0 {
            return Color::default();
        }
    
        if let Some(record) = world.hit(ray, 0.001, f64::MAX) {
            let emitted = record.material.emit(record.u, record.v, &record.point);
            
            if let Some(scatter) = record.material.scatter(ray, &record) {
                return emitted + Self::get_color(&scatter.ray, background, world, depth - 1) * scatter.attenuation;
            }
    
            return emitted;
        }
    
        **background
    }
}
