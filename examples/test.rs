use rust_raytracing::materials::{lambertian::Lambertian, metal::Metal, dielectric::Dielectric};
use rust_raytracing::surfaces::sphere::Sphere;
use rust_raytracing::surfaces::hittable::*;
use rust_raytracing::utils::color::Color;
use rust_raytracing::utils::vec3::Vec3;
use rust_raytracing::camera::Camera;
use rust_raytracing::ray::{Ray};

use rand::distributions::{Distribution, Uniform};

use std::sync::{Arc};
use std::thread;
use std::f64;

fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();
    let range = Uniform::from(0.0..1.0);
    let mut objects = HittableList::new();
    
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(
            Color::new(0.5, 0.5, 0.5),
        ))
    )));

    for a in -11..11 {
        for b in -11..11 {
            let material = range.sample(&mut rng);
            let center = Vec3::new(
                a as f64 + range.sample(&mut rng) * 0.9,
                0.2,
                b as f64 + range.sample(&mut rng) * 0.9
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() <= 0.9 {
                continue;
            }

            if material < 0.7 {
                let albedo = Color::random() * Color::random();
                objects.add(Arc::new(Sphere::new(
                    center,
                    0.2,
                    Arc::new(Lambertian::new(
                        albedo
                    ))
                )));
            }
            else if material < 0.95 {
                let albedo = Color::random();
                let fuzz = range.sample(&mut rng);
                objects.add(Arc::new(Sphere::new(
                    center,
                    0.2,
                    Arc::new(Metal::new(
                        albedo,
                        fuzz
                    ))
                )));
            }
            else {
                objects.add(Arc::new(Sphere::new(
                    center,
                    0.2,
                    Arc::new(Dielectric::new(
                        1.5
                    ))
                )));
            }
        }
    }

    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(
            1.5
        ))
    )));

    objects.add(Arc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(
            Color::new(0.4, 0.2, 0.1)
        ))
    )));

    objects.add(Arc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(
            Color::new(0.7, 0.6, 0.5),
            0.0
        ))
    )));

    objects
}

fn get_color(ray: &Ray, object: Arc<dyn Hittable>, depth: i32) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    if let Some(record) = object.hit(&ray, 0.001, f64::INFINITY) {
        if let Some(scattered) = record.material.scatter(&ray, &record) {
            return get_color(&scattered.ray, object, depth-1) * scattered.attenuation;
        }
        return Color::default();
    }

    let direction = ray.direction.unit_vector();
    let time = 0.5 * (direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - time) + Color::new(0.5, 0.7, 1.0) * time
}

fn main() {
    let witdth = 2000;
    let height = 1000;
    let samples = 70;
    let depth = 30;
    
    let camera = Arc::new(Camera::new(
        Vec3::new(13.0, 2.0, 2.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        witdth as f64 / height as f64
    ));
    
    let objects = Arc::new(random_scene());
    let mut threads = Vec::new();
    
    for row in 0..height {
        eprintln!("launching: {}%", (row as f64 / height as f64 * 100.0) as i32);
        let camera = Arc::clone(&camera);
        let world = objects.clone();
        let handle = thread::spawn(move || {
            let mut result = Vec::new();
            let mut rng = rand::thread_rng();
            let range = Uniform::from(0.0..1.0);
            for col in 0..witdth {
                let mut color = Color::default();
                for _ in 0..samples {
                    let x = (col as f64 + range.sample(&mut rng)) / witdth as f64;
                    let y = (height as f64 - (row as f64 + range.sample(&mut rng))) / height as f64;
                    
                    let ray = camera.get_ray(x, y);
                    color = color + get_color(&ray, world.clone(), depth);
                }
                result.push(color);
            }
            eprintln!("Row {} done", row);
            result
        });
        threads.push(handle);
    }
    
    let mut idx = 0.0;
    println!("P3\n{} {}\n255", witdth, height);
    for thread in threads {
        eprintln!("joining: {}%", (idx/height as f64 * 100.0) as i32);
        for px in thread.join().unwrap() {
            println!("{}", px.to_rgb_with_samples(samples));
        }

        idx += 1.0;
    }
}