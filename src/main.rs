mod ray;
mod vec3;

#[macro_use] extern crate auto_ops;
use vec3::{Vec3, dot, cross};
use ray::Ray;
use std::fs::File;
use std::io::{Write, BufWriter};

fn ray_color(r: &Ray) -> Vec3 {
    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, r) {
       return Vec3::new(1.0, 0.0, 0.0)
    }

    let t = 0.5 * (r.direction.unit().y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin - center;
    let a = dot(r.direction, r.direction);
    let b = 2.0 * dot(oc, r.direction);
    let c = dot(oc, oc) - radius * radius;
    let discriminant = b*b - 4.0*a*c;
    discriminant > 0.0
}

fn main() -> std::io::Result<()> {

    const ASPECT_RATIO: f64 = 16.0/9.0;
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0,0.0,0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut out = BufWriter::new(File::create("img.ppm")?);
    out.write_all(format!("P3\n{} {}\n255\n",WIDTH, HEIGHT).as_bytes())?;

    for row in 0..HEIGHT {
        eprintln!("Lines remaining {}", HEIGHT-row);
        for col in 0..WIDTH {
            let v = row as f64 / (HEIGHT - 1) as f64;
            let u = col as f64 / (WIDTH - 1) as f64;
            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            out.write_all(format!("{}\n", ray_color(&r)).as_bytes())?;
        }
    }
    Ok(())
}
