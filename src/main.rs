use image::{GenericImageView, Pixel};
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
 let mut file = File::create("testpic.npf")?;
 let img = image::open("testpic.png").map_err(|e| {
     std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
 })?;
 let (width, height) = img.dimensions();

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

