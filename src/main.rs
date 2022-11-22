use bevy::prelude::*;

mod debug;
mod physics;
mod camera;
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
