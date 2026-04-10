use rand::{Rng, RngExt};

pub fn random_number(min: f32, max: f32) -> f32 {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}
