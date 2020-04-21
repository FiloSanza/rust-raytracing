use super::surfaces::hittable::{Hittable, HittableList};
use super::ray::{Ray};
use super::utils::color::Color;
use super::camera::Camera;

use std::sync::Arc;
use std::cmp::min;
use std::thread;
use std::f64;

use rand::distributions::{Distribution, Uniform};

pub struct Scene {
    camera: Camera,
    objects: HittableList,
}

impl Scene {
    pub fn new(camera: Camera, objects: HittableList) -> Self {
        Self {
            camera,
            objects: objects,
        }
    }

    pub fn add_object(&mut self, obj: Arc<dyn Hittable>) {
        self.objects.add(obj)
    }

    pub fn add_object_list(&mut self, list: HittableList) {
        for obj in list.objects {
            self.objects.add(obj);
        }
    }

    pub fn render(self, width: usize, height: usize, samples: usize, depth: i32, threads: usize) -> Vec<Color> {
        let camera = Arc::new(self.camera);
        let mut handles = Vec::new();
        let mut image = Vec::new();
        let objects = Arc::new(self.objects);
        let chunk = (height as f64 / threads as f64).ceil() as usize;

        for idx in 0..threads {
            let camera = camera.clone();
            let objects = objects.clone();
            
            eprintln!("launching: {}%", (idx as f64 / threads as f64 * 100.0) as i32);
            
            let handle = thread::spawn(move || {
                let mut result = Vec::new();
                let mut rng = rand::thread_rng();
                let range = Uniform::from(0.0..1.0);
                
                for row in (chunk * idx)..min(chunk * (1 + idx), height) {
                    for col in 0..width {
                        let mut color = Color::default();
                        for _ in 0..samples {
                            let x = (col as f64 + range.sample(&mut rng)) / width as f64;
                            let y = (height as f64 - (row as f64 + range.sample(&mut rng))) / height as f64;
                            
                            let ray = camera.get_ray(x, y);
                            color = color + Self::_get_color(&ray, objects.clone(), depth);
                        }
                        result.push(color);
                    }
                }

                eprintln!("Done {}", idx);

                result
            });

            handles.push(handle);
        }

        let mut count = 0.0;
        for handle in handles {
            eprintln!("joining: {}%", (count as f64 / threads as f64 * 100.0) as i32);
            for px in handle.join().unwrap() {
                image.push(px);
            }
            count += 1.0;
        }

        image
    }

    fn _get_color(ray: &Ray, objects: Arc<HittableList>, depth: i32) -> Color {
        if depth <= 0 {
            return Color::default();
        }
    
        if let Some(record) = objects.hit(&ray, 0.001, f64::INFINITY) {
            if let Some(scattered) = record.material.scatter(&ray, &record) {
                return Self::_get_color(&scattered.ray, objects, depth-1) * scattered.attenuation;
            }
            return Color::default();
        }
    
        let direction = ray.direction.unit_vector();
        let time = 0.5 * (direction.y + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - time) + Color::new(0.5, 0.7, 1.0) * time
    }
}