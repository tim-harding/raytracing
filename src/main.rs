mod image;
mod material;
mod object;
mod ray;
mod shared;
mod world;

use material::Material;
use object::Hit;
use shared::random;

use glam::{Vec3, Vec4};
use image::{Error, Image, CHUNK_DIM};
use ray::Ray;
use world::World;

const W: usize = 1024;
const H: usize = 512;

fn main() -> Result<(), Error> {
    let world = World::test_scene();
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
                const SAMPLES: usize = 16;
                let mut acc = Vec4::ZERO;
                for _ in 0..SAMPLES {
                    let u = ((x_base + x_offset) as f32 + random()) / W as f32;
                    let v = ((y_base + y_offset) as f32 + random()) / H as f32;
                    let dir = lower_left + horizontal * u + vertical * v;
                    let ray = Ray::new(origin, dir);
                    acc += ray_color(ray, &world) / SAMPLES as f32;
                }
                chunk[y_offset][x_offset] = acc;
            }
        }
    }
    image.save("render.exr")?;
    Ok(())
}

fn ray_color(ray: Ray, world: &World) -> Vec4 {
    let hit = world
        .objects
        .iter()
        .fold(None, |closest_so_far: Option<Hit>, &sphere| {
            let hit = sphere.hit(ray);
            match closest_so_far {
                Some(closest_so_far) => match hit {
                    Some(hit) => {
                        if closest_so_far.distance < hit.distance {
                            Some(closest_so_far)
                        } else {
                            Some(hit)
                        }
                    }
                    None => Some(closest_so_far),
                },
                None => hit,
            }
        });
    match hit {
        Some(hit) => {
            let material = world.materials[hit.material];
            let scatter = material.scatter(ray, hit);
            if scatter.attenuation == Vec3::ZERO {
                Vec4::new(0.0, 0.0, 0.0, 1.0)
            } else {
                ray_color(scatter.next_ray, world) * scatter.attenuation.extend(1.0)
            }
        }
        None => {
            let t = 0.5 * ray.direction.x + 0.5;
            let t = t * t;
            let t = t * t;
            let warm = Vec3::new(2.0, 1.8, 1.6);
            let cool = Vec3::new(0.0, 0.2, 0.3);
            let color = cool.lerp(warm, t);
            color.extend(1.0)
        }
    }
}
