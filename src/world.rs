use glam::Vec3;

use crate::{
    material::{Lambert, MaterialKind},
    object::Sphere,
};

pub struct World {
    pub objects: Vec<Sphere>,
    pub materials: Vec<MaterialKind>,
}

impl World {
    pub fn test_scene() -> Self {
        Self {
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
                Sphere {
                    center: Vec3::new(-1.0, 0.0, -1.0),
                    radius: 0.5,
                    material: 0,
                },
                Sphere {
                    center: Vec3::new(1.0, 0.0, -1.0),
                    radius: 0.5,
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
        }
    }
}
