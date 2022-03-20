mod image;

use image::{Error, Image};

fn main() -> Result<(), Error> {
    let image = Image::new(1024, 1024);
    image.save("render.exr")?;
    Ok(())
}
