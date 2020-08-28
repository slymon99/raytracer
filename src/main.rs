mod camera;
mod hittable;
mod ray;
mod vec3;

#[macro_use]
extern crate auto_ops;
use crate::hittable::{HittableList, Sphere};
use ray::Ray;
use vec3::Vec3;
use crate::camera::Camera;
use rand::random;

fn ray_color(r: &Ray, world: &HittableList) -> Vec3 {
    if let Some(hit_data) = world.hit(r, 0.0, f64::INFINITY) {
        return 0.5 * (hit_data.normal + Vec3::new(1.0, 1.0, 1.0));
    }
    let t = 0.5 * (r.direction.unit().y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() -> std::io::Result<()> {
    // Image

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;

    // World

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

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
                pixel_color = pixel_color + ray_color(&r, &world);
            }
            out.put_pixel(col, HEIGHT - row - 1, (pixel_color / SAMPLES_PER_PIXEL as f64).to_pixel());
        }
    }
    out.save("res.png");
    Ok(())
}
