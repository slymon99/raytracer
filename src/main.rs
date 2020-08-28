mod hittable;
mod ray;
mod vec3;

#[macro_use]
extern crate auto_ops;
use crate::hittable::{HittableList, Sphere};
use ray::Ray;
use vec3::Vec3;

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

    // World

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut out = image::ImageBuffer::new(WIDTH, HEIGHT);
    for row in 0..HEIGHT {
        eprintln!("Lines remaining {}", HEIGHT - row);
        for col in 0..WIDTH {
            let v = row as f64 / (HEIGHT - 1) as f64;
            let u = col as f64 / (WIDTH - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            out.put_pixel(col, HEIGHT - row - 1, ray_color(&r, &world).to_pixel());
        }
    }
    out.save("res.png");
    Ok(())
}
