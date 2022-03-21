mod image;
mod ray;

use glam::Vec3;
use image::{Error, Image, Rgba, CHUNK_DIM};
use ray::Ray;

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
    let origin = Vec3::new(0.0, 0.0, 0.0);
    for (i, chunk) in image.chunks_mut().enumerate() {
        let y_base = (i / CHUNK_DIM) * CHUNK_DIM;
        let x_base = (i % CHUNK_DIM) * CHUNK_DIM;
        for y_offset in 0..CHUNK_DIM {
            for x_offset in 0..CHUNK_DIM {
                let u = (x_base + x_offset) as f32 / W as f32;
                let v = (y_base + y_offset) as f32 / H as f32;
                let dir = lower_left + horizontal * u + vertical * v;
                let ray = Ray::new(origin, dir);
                chunk[y_offset][x_offset] = vector_color(ray)
            }
        }
    }
    image.save("render.exr")?;
    Ok(())
}

fn vector_color(ray: Ray) -> Rgba {
    let spheres = vec![
        Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        },
        Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
        },
    ];
    let hit = spheres.iter().find_map(|&sphere| sphere.hit(ray));
    match hit {
        Some(hit) => {
            (0.5 * hit.normal + Vec3::new(0.5, 0.5, 0.5)).into()
        }
        None => {
            let t = 0.5 * ray.direction.normalize().y + 0.5;
            Rgba::rgb(1.0 - t + 0.5 * t, 1.0 - t + 0.7 * t, 1.0)
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn hit(&self, ray: Ray) -> Option<SphereHit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let distance = (-b - discriminant.sqrt()) / 2.0 / a;
            let normal = (ray.at(distance) - self.center).normalize();
            Some(SphereHit {
                normal, distance
            })
        } else {
            None
        }
    }
}

struct SphereHit {
    pub normal: Vec3,
    pub distance: f32,
}