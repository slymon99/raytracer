mod ray;
mod vec3;

#[macro_use] extern crate auto_ops;
use vec3::Vec3;
use ray::Ray;
use std::fs::File;
use std::io::Write;

fn ray_color(r: &Ray) -> Vec3 {
    let t = 0.5 * (r.direction.unit().y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() -> std::io::Result<()> {

    const ASPECT_RATIO: f64 = 16.0/9.0;
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0,0.0,0.0);
    let horizontal = Vec3::new(viewport_height, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = &origin - &horizontal / 2.0 - &vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut out = File::create("img.ppm")?;
    out.write(format!("P3\n{} {}\n255\n",WIDTH, HEIGHT).as_bytes())?;

    for row in 0..HEIGHT {
        eprintln!("Lines remaining {}", HEIGHT-row);
        for col in 0..WIDTH {
            let u = row as f64 / (HEIGHT - 1) as f64;
            let v = col as f64 / (WIDTH - 1) as f64;
            let r = Ray::new(origin, &lower_left_corner + u*&horizontal + v*&vertical - &origin);
            out.write(format!("{}\n", ray_color(&r)).as_bytes());
        }
    }
    Ok(())
}
