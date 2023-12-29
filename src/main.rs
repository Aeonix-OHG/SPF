use image::{GenericImageView, Pixel};
use std::fs::File;
use std::io::Write;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let out_path = &args[2];
    let img = image::open(path).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    })?;
    let (width, height) = img.dimensions();
    let mut file = File::create(out_path)?;
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let (r, g, b, _) = pixel.channels4();
            write!(file, "{}s{}s{}p", r, g, b)?;
        }
        file.write_all(b"\n")?;
    }

    Ok(())
}

