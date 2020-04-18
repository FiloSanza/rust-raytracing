use rust_raytracing::materials::{lambertian::Lambertian, metal::Metal, dielectric::Dielectric};
use rust_raytracing::surfaces::sphere::Sphere;
use rust_raytracing::surfaces::hittable::*;
use rust_raytracing::utils::color::Color;
use rust_raytracing::utils::vec3::Vec3;
use rust_raytracing::camera::Camera;
use rust_raytracing::scene::Scene;

use rand::distributions::{Distribution, Uniform};

use std::sync::{Arc};
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

fn main() {
    let witdth = 4096;
    let height = 2160;
    let samples = 70;
    let depth = 30;
    
    let scene = Scene::new(
        Camera::new(
            Vec3::new(13.0, 2.0, 2.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            witdth as f64 / height as f64,
            10.0,
            1.0
        ),
        random_scene()
    );    
    
    println!("P3\n{} {}\n255", witdth, height);
    for px in scene.render(witdth, height, samples, depth, 500){
        println!("{}", px.to_rgb_with_samples(samples as i32));
    }
}