use glam::Vec3;

use crate::{
    object::Hit,
    ray::Ray,
    shared::{random, random_on_unit_sphere},
};

#[derive(Debug, Clone, Copy)]
pub enum MaterialKind {
    Lambert(Lambert),
}

impl Material for MaterialKind {
    fn scatter(&self, ray: Ray, hit: Hit) -> Scatter {
        match self {
            MaterialKind::Lambert(lambert) => lambert.scatter(ray, hit),
        }
    }
}

pub trait Material {
    fn scatter(&self, ray: Ray, hit: Hit) -> Scatter;
}

#[derive(Debug, Clone, Copy)]
pub struct Scatter {
    pub attenuation: Vec3,
    pub next_ray: Ray,
}

impl Default for Scatter {
    fn default() -> Self {
        Self {
            attenuation: Vec3::ZERO,
            next_ray: Default::default(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Lambert {
    pub albedo: Vec3,
}

impl Material for Lambert {
    fn scatter(&self, _ray: Ray, hit: Hit) -> Scatter {
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
