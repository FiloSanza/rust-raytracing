#![allow(unused_imports)]
#![allow(dead_code)]

use rust_raytracing::objects::{sphere::Sphere, moving_sphere::MovingSphere, rectangles::*, cube::Cube, constant_medium::ConstantMedium};
use rust_raytracing::material::{light::Light, dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use rust_raytracing::textures::{CheckerTexture, ConstantTexture, NoiseTexture, ImageTexture};
use rust_raytracing::hittable::{Hittable, HittableList, FlipFace, RotateY, Translate};
use rust_raytracing::hittable::bounding::BvhNode;
use rust_raytracing::camera::camera::Camera;
use rust_raytracing::utils::color::Color;
use rust_raytracing::utils::vec3::Vec3;
use rust_raytracing::utils::ray::Ray;
use rust_raytracing::scene::Scene;

use std::io;
use std::f64;
use std::sync::Arc;
use std::io::Write;

use rand::distributions::{Distribution, Uniform};
use rand::Rng;

use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

fn random_scene() -> Vec<Arc<dyn Hittable>> {
    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();
    let mut rng = rand::thread_rng();
    let range = Uniform::from(0.0..1.0);

    world.push(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Arc::new(
            CheckerTexture::new(
                Arc::new(ConstantTexture::new(Color::new(0.2, 0.3, 0.1))),
                Arc::new(ConstantTexture::new(Color::new(0.9, 0.9, 0.9))),
            )
        )))
    )));

    for a in -10..10 {
        for b in -10..10 {
            let mat = range.sample(&mut rng);
            let center = Vec3::new(
                a as f64 + 0.9 * range.sample(&mut rng),
                0.2,
                b as f64 + 0.9 * range.sample(&mut rng),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if mat < 0.8 {
                    let color = Color::random() * Color::random();
                    world.push(Arc::new(MovingSphere::new(
                        center,
                        center + Vec3::new(0.0, rng.gen_range(0.0, 0.5), 0.0),
                        0.0,
                        1.0,
                        0.2,
                        Arc::new(Lambertian::new(Arc::new(
                            ConstantTexture::new(color)
                        )))
                    )));
                }
                else if mat < 0.95 {
                    let color = Color::random();
                    let fuzz = range.sample(&mut rng);

                    world.push(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Metal::new(
                            color,
                            fuzz
                        ))
                    )));
                }
                else{
                    world.push(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Dielectric::new(
                            1.5
                        ))
                    )));
                }
            }
        }
    }

    world.push(Arc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(
            1.5
        ))
    )));

    world.push(Arc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Arc::new(
            ConstantTexture::new(Color::new(0.4, 0.2, 0.1))
        )))
    )));
    world.push(Arc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(
            Color::new(0.7, 0.6, 0.5),
            0.0
        ))
    )));

    world
}

fn perlin_scene() -> Vec<Arc<dyn Hittable>> {
    let mut scene: Vec<Arc<dyn Hittable>> = vec![];

    let perlin = Arc::new(NoiseTexture::new(4.0));

    scene.push(Arc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(Lambertian::new(perlin.clone())))));
    scene.push(Arc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Arc::new(Lambertian::new(perlin.clone())))));

    let light = Arc::new(Light::new(Arc::new(ConstantTexture::new(Color::new(4.0, 4.0, 4.0)))));

    scene.push(Arc::new(Sphere::new(Vec3::new(0.0, 7.0, 0.0), 2.0, light.clone())));
    scene.push(Arc::new(XYRectangle::new(3.0, 5.0, 1.0, 3.0, -2.0, light.clone())));

    scene
}

fn cornell_box() -> Vec<Arc<dyn Hittable>> {
    let mut objects: Vec<Arc<dyn Hittable>> = vec![];

    let red = Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Color::new(0.65, 0.05, 0.05)))));
    let white = Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Color::new(0.73, 0.73, 0.73)))));
    let green = Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Color::new(0.12, 0.45, 0.15)))));
    let light = Arc::new(Light::new(Arc::new(ConstantTexture::new(Color::new(7.0, 7.0, 7.0)))));

    objects.push(Arc::new(FlipFace::new(Arc::new(YZRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, green.clone())))));
    objects.push(Arc::new(YZRectangle::new(0.0, 555.0, 0.0, 555.0, 0.0, red.clone())));
    objects.push(Arc::new(XZRectangle::new(113.0, 443.0, 127.0, 432.0, 554.0, light.clone())));
    objects.push(Arc::new(FlipFace::new(Arc::new(XYRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())))));
    objects.push(Arc::new(XZRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    objects.push(Arc::new(XZRectangle::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())));
    objects.push(Arc::new(FlipFace::new(Arc::new(XYRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())))));

    let mut box1: Arc<dyn Hittable> = Arc::new(Cube::from_vertices(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 330.0, 165.0), white.clone()));
    let mut box2: Arc<dyn Hittable> = Arc::new(Cube::from_vertices(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 165.0, 165.0), white.clone()));
    
    box1 = Arc::new(RotateY::new(box1.clone(), 15.0));
    box2 = Arc::new(RotateY::new(box2.clone(), -18.0));

    box1 = Arc::new(Translate::new(box1.clone(), Vec3::new(256.0, 0.0, 295.0)));
    box2 = Arc::new(Translate::new(box2.clone(), Vec3::new(130.0, 0.0, 65.0)));

    objects.push(Arc::new(ConstantMedium::new(box1.clone(), Arc::new(ConstantTexture::new(Color::new(0.0, 0.0, 0.0))), 0.01)));
    objects.push(Arc::new(ConstantMedium::new(box2.clone(), Arc::new(ConstantTexture::new(Color::new(1.0, 1.0, 1.0))), 0.01)));

    objects
}

fn test_scene() -> Vec<Arc<dyn Hittable>> {
    let mut floor: Vec<Arc<dyn Hittable>> = vec![];
    let mut scene: Vec<Arc<dyn Hittable>> = vec![];
    let mut objects: Vec<Arc<dyn Hittable>> = vec![];
    let mut rng = rand::thread_rng();

    let img = image::open("./res/earthmap.jpg").expect("Could not open the image");

    let boxes = 20;
    
    let ground_color = Arc::new(ConstantTexture::new(Color::new(0.48, 0.83, 0.53)));
    let ground = Arc::new(Lambertian::new(ground_color.clone()));
    
    for i in 0..boxes {
        for j in 0..boxes {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let z1 = z0 + w;
            let y1 = rng.gen_range(1.0, 100.0);
            
            floor.push(Arc::new(Cube::from_vertices(Vec3::new(x0, y0, z0), Vec3::new(x1, y1, z1), ground.clone())));
        }
    }
    
    let light = Arc::new(Light::new(Arc::new(ConstantTexture::new(Color::new(7.0, 7.0, 7.0)))));
    objects.push(Arc::new(XZRectangle::new(123.0, 423.0, 147.0, 412.0, 554.0, light.clone())));
    

    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Color::new(0.7, 0.3, 0.1)))));

    objects.push(Arc::new(MovingSphere::new(center1, center2, 0.0, 1.0, 50.0, moving_sphere_material.clone())));
    
    objects.push(Arc::new(Sphere::new(Vec3::new(260.0, 150.0, 45.0), 50.0, Arc::new(Dielectric::new(1.5)))));
    objects.push(Arc::new(Sphere::new(Vec3::new(0.0, 150.0, 145.0), 50.0, Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.5)))));

    let boundary1 = Arc::new(Sphere::new(Vec3::new(360.0, 150.0, 145.0), 70.0, Arc::new(Dielectric::new(1.5))));

    objects.push(boundary1.clone());
    objects.push(Arc::new(ConstantMedium::new(boundary1.clone(), Arc::new(ConstantTexture::new(Color::new(0.2, 0.4, 0.9))), 0.2)));
    
    let boundary2 = Arc::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 5000.0, Arc::new(Dielectric::new(1.5))));

    objects.push(Arc::new(ConstantMedium::new(boundary2.clone(), Arc::new(ConstantTexture::new(Color::new(1.0, 1.0, 1.0))), 0.0001)));

    let earth_material = Arc::new(Lambertian::new(Arc::new(ImageTexture::new(img))));
    objects.push(Arc::new(Sphere::new(Vec3::new(400.0, 200.0, 400.0), 100.0, earth_material.clone())));

    let noise_texture = Arc::new(NoiseTexture::new(0.1));
    objects.push(Arc::new(Sphere::new(Vec3::new(220.0, 280.0, 300.0), 80.0, Arc::new(Lambertian::new(noise_texture.clone())))));

    let mut boxes2: Vec<Arc<dyn Hittable>> = vec![];
    
    let white = Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Color::new(1.0, 1.0, 1.0)))));
    
    let cont = 1000;
    for _ in 0..cont {
        boxes2.push(Arc::new(Sphere::new(Vec3::random_range(0.0, 165.0), 10.0, white.clone())));
    }
    
    objects.push(Arc::new(
        Translate::new(Arc::new(RotateY::new(Arc::new(BvhNode::new(&mut boxes2, 0, cont, 0.0, 1.0)), 15.0)), 
        Vec3::new(-100.0, 270.0, 395.0)
    )));
    
    let sz_floor = floor.len();
    let sz_objects = objects.len();
    scene.push(Arc::new(BvhNode::new(&mut floor, 0, sz_floor, 0.0, 0.1)));
    scene.push(Arc::new(BvhNode::new(&mut objects, 0, sz_objects, 0.0, 0.1)));
        
    scene
}

fn main() {
    let start = std::time::Instant::now();

    let width: usize = 600;
    let height: usize = 600;
    let samples: usize = 10000; 
    let depth: i32 = 50; 
    let threads: usize = 100; 
    let origin: Vec3 = Vec3::new(478.0, 278.0, -600.0);
    let look_at: Vec3 = Vec3::new(278.0, 278.0, 0.0);
    let aperture = 0.0;
    let dist_to_focus = 10.0;
    let background = Color::default();

    let camera = Camera::new(
        origin,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        (width as f64) / (height as f64),
        aperture,
        dist_to_focus,
        0.0,
        1.0
    );

    let mut world = test_scene();
    let sz = world.len();

    let world = Arc::new(BvhNode::new(&mut world, 0, sz, 0.0, 1.0));

    let scene = Scene::new(world.clone(), camera, background);
    let pixels = scene.render(width, height, samples, depth, threads);

    let mut buffer = image::ImageBuffer::new(width as u32, height as u32);

    let mut idx = 0;
    for (_, _, px) in buffer.enumerate_pixels_mut() {
        let (r, g, b) = pixels[idx].to_rgb_with_samples(samples as i32);
        idx += 1;

        *px = image::Rgb([r, g, b]);
    }

    buffer.save("test.png").unwrap();

    eprintln!("DONE: {}ms", start.elapsed().as_millis());
}