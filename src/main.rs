mod image;
mod ray;

use rand::prelude::*;

use glam::{Vec3, Vec4};
use image::{Error, Image, CHUNK_DIM};
use ray::Ray;

const W: usize = 1024;
const H: usize = 512;

fn main() -> Result<(), Error> {
    let mut rand = rand::rngs::SmallRng::seed_from_u64(7);
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
                chunk[y_offset][x_offset] = ray_color(ray, &mut rand);
            }
        }
    }
    image.save("render.exr")?;
    Ok(())
}

fn ray_color(ray: Ray, rand: &mut SmallRng) -> Vec4 {
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
            let new_direction = hit.normal + rand_on_unit_sphere(rand);
            Vec4::new(0.5, 0.5, 0.5, 1.0) * ray_color(Ray::new(hit.point, new_direction), rand)
        }
        None => {
            let t = 0.5 * ray.direction.normalize().y + 0.5;
            Vec4::new(1.0 - t + 0.5 * t, 1.0 - t + 0.7 * t, 1.0, 1.0)
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
        if discriminant > 0.0001 {
            let distance = (-b - discriminant.sqrt()) / 2.0 / a;
            let point = ray.at(distance);
            if distance > 0.0 {
                let normal = (point - self.center).normalize();
                Some(SphereHit {
                    point,
                    normal,
                    distance,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn rand_on_unit_sphere(rand: &mut SmallRng) -> Vec3 {
    let mut v = candidate_unit_vector(rand);
    while v.length_squared() > 1.0 {
        v = candidate_unit_vector(rand);
    }
    v.normalize()
}

fn candidate_unit_vector(rand: &mut SmallRng) -> Vec3 {
    Vec3::new(rand.gen(), rand.gen(), rand.gen()) * 2.0 - 1.0
}

struct SphereHit {
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f32,
}
