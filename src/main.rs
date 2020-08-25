mod ray;
mod vec3;

use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    const WIDTH: u32 = 512;
    const HEIGHT: u32 = 512;

    let mut out = File::create("img.ppm")?;
    out.write(format!("P3\n{} {}\n255\n",WIDTH, HEIGHT).as_bytes())?;

    for row in 0..HEIGHT {
        eprintln!("Lines remaining {}", HEIGHT-row);
        for col in 0..WIDTH {
            let v = vec3::Vec3::new(row as f64 / HEIGHT as f64, col as f64 / WIDTH as f64, 0.25);
            out.write(format!("{}\n", v).as_bytes());
        }
    }
    Ok(())
}
