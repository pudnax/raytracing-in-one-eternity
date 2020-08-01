mod color;
mod vec3;

use color::Color;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    let mut writer = std::io::BufWriter::new(std::io::stdout());
    let mut err_writer = std::io::BufWriter::new(std::io::stderr());

    writer.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())?;

    for j in (0..=(IMAGE_HEIGHT - 1)).rev() {
        for i in 0..IMAGE_WIDTH {
            err_writer.write_all(format!("\rScanlines remaning: {} ", j).as_bytes())?;

            let color = Color::new(
                i as f64 / (IMAGE_WIDTH as f64 - 1.),
                j as f64 / (IMAGE_HEIGHT as f64 - 1.),
                0.25,
            );

            // color::wrile_color(&mut writer, color)?;
            color.write_to(&mut writer)?;
        }
    }

    err_writer.write(b"\nDone.\n")?;
    Ok(())
}
