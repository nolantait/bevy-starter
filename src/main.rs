use bevy::prelude::*;
use rand::Rng;

mod debug;
mod physics;
mod camera;
mod boids;
mod input;
mod game;

use crate::debug::DebugPlugin;
use crate::game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(GamePlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}

pub fn random_number(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(min..max);
}
