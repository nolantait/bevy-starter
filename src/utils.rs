use rand::Rng;

pub fn random_number(min: f32, max: f32) -> f32 {
    let mut rng = rand::rng();
    return rng.random_range(min..max);
}
