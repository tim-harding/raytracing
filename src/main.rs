mod image;
mod ray;

use std::cell::RefCell;

use rand::prelude::*;

use glam::{Vec3, Vec4};
use image::{Error, Image, CHUNK_DIM};
use ray::Ray;

const W: usize = 1024;
const H: usize = 512;

thread_local! {
    static RANDOM: RefCell<SmallRng> = RefCell::new(rand::rngs::SmallRng::seed_from_u64(7));
}

fn main() -> Result<(), Error> {
    let world = World {
        objects: vec![
            Sphere {
                center: Vec3::new(0.0, 0.0, -1.0),
                radius: 0.5,
                material: 0,
            },
            Sphere {
                center: Vec3::new(0.0, -100.5, -1.0),
                radius: 100.0,
                material: 1,
            },
        ],
        materials: vec![
            MaterialKind::Lambert(Lambert {
                albedo: Vec3::new(0.8, 0.2, 0.2),
            }),
            MaterialKind::Lambert(Lambert {
                albedo: Vec3::new(0.2, 0.8, 0.2),
            }),
        ],
    };
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
    let hit = world.objects.iter().find_map(|&sphere| sphere.hit(ray));
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
            let t = 0.5 * ray.direction.normalize().y + 0.5;
            Vec4::new(1.0 - t + 0.5 * t, 1.0 - t + 0.7 * t, 1.0, 1.0)
        }
    }
}

struct World {
    objects: Vec<Sphere>,
    materials: Vec<MaterialKind>,
}

#[derive(Debug, Clone, Copy)]
enum MaterialKind {
    Lambert(Lambert),
}

impl Material for MaterialKind {
    fn scatter(&self, ray: Ray, hit: Hit) -> Scatter {
        match self {
            MaterialKind::Lambert(lambert) => lambert.scatter(ray, hit),
        }
    }
}

trait Material {
    fn scatter(&self, ray: Ray, hit: Hit) -> Scatter;
}

#[derive(Debug, Clone, Copy)]
struct Scatter {
    attenuation: Vec3,
    next_ray: Ray,
}

impl Default for Scatter {
    fn default() -> Self {
        Self {
            attenuation: Vec3::ZERO,
            next_ray: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f32,
    pub material: usize,
}

#[derive(Debug, Default, Clone, Copy)]
struct Lambert {
    albedo: Vec3,
}

impl Material for Lambert {
    fn scatter(&self, ray: Ray, hit: Hit) -> Scatter {
        const CANCEL_PROBABILITY: f32 = 63.0 / 64.0;
        if random() > CANCEL_PROBABILITY {
            Scatter {
                attenuation: Vec3::ZERO,
                ..Default::default()
            }
        } else {
            let new_direction = hit.normal + random_on_unit_sphere();
            let new_ray = Ray::new(hit.point, new_direction);
            Scatter {
                attenuation: self.albedo / CANCEL_PROBABILITY,
                next_ray: new_ray,
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: usize,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: usize) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn hit(&self, ray: Ray) -> Option<Hit> {
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
                Some(Hit {
                    point,
                    normal,
                    distance,
                    material: self.material,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn random_on_unit_sphere() -> Vec3 {
    let mut v = candidate_unit_vector();
    while v.length_squared() > 1.0 {
        v = candidate_unit_vector();
    }
    v.normalize()
}

fn candidate_unit_vector() -> Vec3 {
    random_vector() * 2.0 - 1.0
}

fn random() -> f32 {
    RANDOM.with(|r| r.borrow_mut().gen())
}

fn random_vector() -> Vec3 {
    Vec3::new(random(), random(), random())
}
