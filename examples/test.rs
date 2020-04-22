#![allow(unused_imports)]
#![allow(dead_code)]

use rust_raytracingv2::objects::{sphere::Sphere, moving_sphere::MovingSphere, rectangles::*, cube::Cube, constant_medium::ConstantMedium};
use rust_raytracingv2::material::{light::Light, dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use rust_raytracingv2::textures::{CheckerTexture, ConstantTexture, NoiseTexture, ImageTexture};
use rust_raytracingv2::hittable::{Hittable, HittableList, FlipFace, RotateY, Translate};
use rust_raytracingv2::hittable::bounding::BvhNode;
use rust_raytracingv2::camera::camera::Camera;
use rust_raytracingv2::utils::color::Color;
use rust_raytracingv2::utils::vec3::Vec3;
use rust_raytracingv2::utils::ray::Ray;

use std::io;
use std::f64;
use std::rc::Rc;
use std::io::Write;

use rand::distributions::{Distribution, Uniform};
use rand::Rng;

use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

fn random_scene() -> Vec<Rc<dyn Hittable>> {
    let mut world: Vec<Rc<dyn Hittable>> = Vec::new();
    let mut rng = rand::thread_rng();
    let range = Uniform::from(0.0..1.0);

    world.push(Rc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(Rc::new(
            CheckerTexture::new(
                Rc::new(ConstantTexture::new(Color::new(0.2, 0.3, 0.1))),
                Rc::new(ConstantTexture::new(Color::new(0.9, 0.9, 0.9))),
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
                    world.push(Rc::new(MovingSphere::new(
                        center,
                        center + Vec3::new(0.0, rng.gen_range(0.0, 0.5), 0.0),
                        0.0,
                        1.0,
                        0.2,
                        Rc::new(Lambertian::new(Rc::new(
                            ConstantTexture::new(color)
                        )))
                    )));
                }
                else if mat < 0.95 {
                    let color = Color::random();
                    let fuzz = range.sample(&mut rng);

                    world.push(Rc::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Metal::new(
                            color,
                            fuzz
                        ))
                    )));
                }
                else{
                    world.push(Rc::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Dielectric::new(
                            1.5
                        ))
                    )));
                }
            }
        }
    }

    world.push(Rc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric::new(
            1.5
        ))
    )));

    world.push(Rc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(Rc::new(
            ConstantTexture::new(Color::new(0.4, 0.2, 0.1))
        )))
    )));
    world.push(Rc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(
            Color::new(0.7, 0.6, 0.5),
            0.0
        ))
    )));

    world
}

fn perlin_scene() -> Vec<Rc<dyn Hittable>> {
    let mut scene: Vec<Rc<dyn Hittable>> = vec![];

    let perlin = Rc::new(NoiseTexture::new(4.0));

    scene.push(Rc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Rc::new(Lambertian::new(perlin.clone())))));
    scene.push(Rc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Rc::new(Lambertian::new(perlin.clone())))));

    let light = Rc::new(Light::new(Rc::new(ConstantTexture::new(Color::new(4.0, 4.0, 4.0)))));

    scene.push(Rc::new(Sphere::new(Vec3::new(0.0, 7.0, 0.0), 2.0, light.clone())));
    scene.push(Rc::new(XYRectangle::new(3.0, 5.0, 1.0, 3.0, -2.0, light.clone())));

    scene
}

fn cornell_box() -> Vec<Rc<dyn Hittable>> {
    let mut objects: Vec<Rc<dyn Hittable>> = vec![];

    let red = Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Color::new(0.65, 0.05, 0.05)))));
    let white = Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Color::new(0.73, 0.73, 0.73)))));
    let green = Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Color::new(0.12, 0.45, 0.15)))));
    let light = Rc::new(Light::new(Rc::new(ConstantTexture::new(Color::new(7.0, 7.0, 7.0)))));

    objects.push(Rc::new(FlipFace::new(Rc::new(YZRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, green.clone())))));
    objects.push(Rc::new(YZRectangle::new(0.0, 555.0, 0.0, 555.0, 0.0, red.clone())));
    objects.push(Rc::new(XZRectangle::new(113.0, 443.0, 127.0, 432.0, 554.0, light.clone())));
    objects.push(Rc::new(FlipFace::new(Rc::new(XYRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())))));
    objects.push(Rc::new(XZRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    objects.push(Rc::new(XZRectangle::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())));
    objects.push(Rc::new(FlipFace::new(Rc::new(XYRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())))));

    let mut box1: Rc<dyn Hittable> = Rc::new(Cube::from_vertices(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 330.0, 165.0), white.clone()));
    let mut box2: Rc<dyn Hittable> = Rc::new(Cube::from_vertices(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 165.0, 165.0), white.clone()));
    
    box1 = Rc::new(RotateY::new(box1.clone(), 15.0));
    box2 = Rc::new(RotateY::new(box2.clone(), -18.0));

    box1 = Rc::new(Translate::new(box1.clone(), Vec3::new(256.0, 0.0, 295.0)));
    box2 = Rc::new(Translate::new(box2.clone(), Vec3::new(130.0, 0.0, 65.0)));

    objects.push(Rc::new(ConstantMedium::new(box1.clone(), Rc::new(ConstantTexture::new(Color::new(0.0, 0.0, 0.0))), 0.01)));
    objects.push(Rc::new(ConstantMedium::new(box2.clone(), Rc::new(ConstantTexture::new(Color::new(1.0, 1.0, 1.0))), 0.01)));

    objects
}

fn test_scene() -> Vec<Rc<dyn Hittable>> {
    let mut floor: Vec<Rc<dyn Hittable>> = vec![];
    let mut scene: Vec<Rc<dyn Hittable>> = vec![];
    let mut objects: Vec<Rc<dyn Hittable>> = vec![];
    let mut rng = rand::thread_rng();

    let img = image::open("./res/earthmap.jpg").expect("Could not open the image");

    let boxes = 20;
    
    let ground_color = Rc::new(ConstantTexture::new(Color::new(0.48, 0.83, 0.53)));
    let ground = Rc::new(Lambertian::new(ground_color.clone()));
    
    for i in 0..boxes {
        for j in 0..boxes {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let z1 = z0 + w;
            let y1 = rng.gen_range(1.0, 100.0);
            
            floor.push(Rc::new(Cube::from_vertices(Vec3::new(x0, y0, z0), Vec3::new(x1, y1, z1), ground.clone())));
        }
    }
    
    let light = Rc::new(Light::new(Rc::new(ConstantTexture::new(Color::new(7.0, 7.0, 7.0)))));
    objects.push(Rc::new(XZRectangle::new(123.0, 423.0, 147.0, 412.0, 554.0, light.clone())));
    

    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Color::new(0.7, 0.3, 0.1)))));

    objects.push(Rc::new(MovingSphere::new(center1, center2, 0.0, 1.0, 50.0, moving_sphere_material.clone())));
    
    objects.push(Rc::new(Sphere::new(Vec3::new(260.0, 150.0, 45.0), 50.0, Rc::new(Dielectric::new(1.5)))));
    objects.push(Rc::new(Sphere::new(Vec3::new(0.0, 150.0, 145.0), 50.0, Rc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.5)))));

    let boundary1 = Rc::new(Sphere::new(Vec3::new(360.0, 150.0, 145.0), 70.0, Rc::new(Dielectric::new(1.5))));

    objects.push(boundary1.clone());
    objects.push(Rc::new(ConstantMedium::new(boundary1.clone(), Rc::new(ConstantTexture::new(Color::new(0.2, 0.4, 0.9))), 0.2)));
    
    let boundary2 = Rc::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 5000.0, Rc::new(Dielectric::new(1.5))));

    objects.push(Rc::new(ConstantMedium::new(boundary2.clone(), Rc::new(ConstantTexture::new(Color::new(1.0, 1.0, 1.0))), 0.0001)));

    let earth_material = Rc::new(Lambertian::new(Rc::new(ImageTexture::new(img))));
    objects.push(Rc::new(Sphere::new(Vec3::new(400.0, 200.0, 400.0), 100.0, earth_material.clone())));

    let noise_texture = Rc::new(NoiseTexture::new(0.1));
    objects.push(Rc::new(Sphere::new(Vec3::new(220.0, 280.0, 300.0), 80.0, Rc::new(Lambertian::new(noise_texture.clone())))));

    let mut boxes2: Vec<Rc<dyn Hittable>> = vec![];
    
    let white = Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Color::new(1.0, 1.0, 1.0)))));
    
    let cont = 1000;
    for _ in 0..cont {
        boxes2.push(Rc::new(Sphere::new(Vec3::random_range(0.0, 165.0), 10.0, white.clone())));
    }
    
    objects.push(Rc::new(
        Translate::new(Rc::new(RotateY::new(Rc::new(BvhNode::new(&mut boxes2, 0, cont, 0.0, 1.0)), 15.0)), 
        Vec3::new(-100.0, 270.0, 395.0)
    )));
    
    let sz_floor = floor.len();
    let sz_objects = objects.len();
    scene.push(Rc::new(BvhNode::new(&mut floor, 0, sz_floor, 0.0, 0.1)));
    scene.push(Rc::new(BvhNode::new(&mut objects, 0, sz_objects, 0.0, 0.1)));
        
    scene
}

fn get_color(ray: &Ray, background: &Color, world: Rc<dyn Hittable>, depth: i32) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    if let Some(record) = world.hit(ray, 0.001, f64::MAX) {
        let emitted = record.material.emit(record.u, record.v, &record.point);
        
        if let Some(scatter) = record.material.scatter(ray, &record) {
            return emitted + get_color(&scatter.ray, background, world, depth - 1) * scatter.attenuation;
        }

        return emitted;
    }

    *background
}

fn main() {
    let start = std::time::Instant::now();

    let width: usize = 600;
    let height: usize = 600;
    let samples: usize = 10000; 
    let depth: i32 = 50; 
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

    let world = Rc::new(BvhNode::new(&mut world, 0, sz, 0.0, 1.0));

    let mut rng = rand::thread_rng();
    let range = Uniform::from(0.0..1.0);

    let mut pixels = vec![];

    for row in (0..height).rev() {
        println!("{:.2}%", (height - row) as f64 / height as f64 * 100.0);
        for col in 0..width {
            let mut color = Color::default();
            for _ in 0..samples {
                let x = (col as f64 + range.sample(&mut rng)) / width as f64;
                let y = (row as f64 + range.sample(&mut rng)) / height as f64;
                
                let ray = camera.get_ray(x, y);
                color = color + get_color(&ray, &background, world.clone(), depth);
            }
            pixels.push(color);
        }
        io::stdout().flush().unwrap();
    }

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