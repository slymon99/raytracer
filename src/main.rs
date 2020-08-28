mod camera;
mod hittable;
mod material;
mod ray;
mod vec3;

#[macro_use]
extern crate auto_ops;
use crate::camera::Camera;
use crate::hittable::{HittableList, Sphere, Hittable};
use crate::material::{scatter, Material};
use rand::{random, Rng};
use ray::Ray;
use std::f64::consts::PI;
use vec3::Vec3;

fn ray_color(r: &Ray, world: &HittableList, depth: u32) -> Vec3 {
    if depth == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    if let Some(hit_data) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some(scatter_data) = scatter(r, &hit_data) {
            return scatter_data.attenuation * ray_color(&scatter_data.scattered, world, depth - 1);
        }
        return Vec3::new(0.0, 0.0, 0.0);
    }
    let t = 0.5 * (r.direction.unit().y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn rand_between(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min, max)
}

pub fn random_unit_vector() -> Vec3 {
    let a = rand_between(0.0, 2.0 * PI);
    let z = rand_between(-1.0, 1.0);
    let r = (1.0 - z * z).sqrt();
    Vec3::new(r * (a.cos()), r * (a.sin()), z)
}

fn main() -> std::io::Result<()> {
    // Image

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const WIDTH: u32 = 2560;
    const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;

    // World

    let mut world = HittableList::new();
    let metal_one = Material::Metal(Vec3::new(0.3, 0.4, 0.6));
    let diffuse_red = Material::Diffuse(Vec3::new(0.7, 0.3, 0.3));
    let focus_sphere = Hittable::Sphere(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        &metal_one));
    let big_sphere = Hittable::Sphere(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            &metal_one));
    // world.add(focus_sphere);
    world.add(big_sphere);

    for idx in 0..5 {
        let radius = 0.2;
        world.add(Hittable::Sphere(Sphere::new(
            Vec3::new((idx as f64 - 2.0) * 0.5, 0.0, -1.0), radius, &metal_one)
        ));
    }

    // Camera
    let cam = Camera::new();

    let mut out = image::ImageBuffer::new(WIDTH, HEIGHT);
    for row in 0..HEIGHT {
        eprintln!("Lines remaining {}", HEIGHT - row);
        for col in 0..WIDTH {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let v_offset: f64 = random();
                let u_offset: f64 = random();
                let v = (row as f64 + v_offset) / (HEIGHT - 1) as f64;
                let u = (col as f64 + u_offset) / (WIDTH - 1) as f64;
                let r = cam.ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world, 50);
            }
            out.put_pixel(
                col,
                HEIGHT - row - 1,
                (pixel_color / SAMPLES_PER_PIXEL as f64).to_pixel(),
            );
        }
    }
    out.save("res.png");
    Ok(())
}
