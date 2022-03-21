mod image;
mod ray;

use glam::Vec3;
use image::{Error, Image, Rgba, CHUNK_DIM};

fn main() -> Result<(), Error> {
    const W: usize = 1024;
    const H: usize = 512;
    let mut image = Image::new(W, H);
    let viewport_height = 2.0f32;
    let viewport_width = 2.0 * image.width() as f32 / image.height() as f32;
    let focal_length = 1.0f32;
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left = Vec3::new(-viewport_width / 2.0, -viewport_height / 2.0, -focal_length);
    for (i, chunk) in image.chunks_mut().enumerate() {
        let y_base = (i / CHUNK_DIM) * CHUNK_DIM;
        let x_base = (i % CHUNK_DIM) * CHUNK_DIM;
        for y_offset in 0..CHUNK_DIM {
            for x_offset in 0..CHUNK_DIM {
                let u = (x_base + x_offset) as f32 / W as f32;
                let v = (y_base + y_offset) as f32 / H as f32;
                let dir = lower_left + horizontal * u + vertical * v;
                chunk[y_offset][x_offset] = vector_color(dir)
            }
        }
    }
    image.save("render.exr")?;
    Ok(())
}

fn vector_color(vector: Vec3) -> Rgba {
    let t = 0.5 * vector.abs().y + 0.5;
    Rgba::rgb(1.0 - t + 0.5 * t, 1.0 - t + 0.7 * t, 1.0)
}
