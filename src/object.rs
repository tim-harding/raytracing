use glam::Vec3;

use crate::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f32,
    pub material: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: usize,
}

impl Sphere {
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
