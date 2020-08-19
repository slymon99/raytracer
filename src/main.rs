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
            out.write(format!("{} {} {}\n", row%255, col%255, 255).as_bytes());
        }
    }
    Ok(())
}
