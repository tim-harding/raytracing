use rand::prelude::*;
use std::cell::RefCell;

use glam::Vec3;

thread_local! {
    static RANDOM: RefCell<SmallRng> = RefCell::new(rand::rngs::SmallRng::seed_from_u64(7));
}

pub fn random_on_unit_sphere() -> Vec3 {
    let mut v = candidate_unit_vector();
    while v.length_squared() > 1.0 {
        v = candidate_unit_vector();
    }
    v.normalize()
}

fn candidate_unit_vector() -> Vec3 {
    random_vector() * 2.0 - 1.0
}

pub fn random() -> f32 {
    RANDOM.with(|r| r.borrow_mut().gen())
}

pub fn random_vector() -> Vec3 {
    Vec3::new(random(), random(), random())
}
