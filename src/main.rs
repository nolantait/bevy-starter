use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod debug;
mod physics;
mod camera;
mod input;
mod game;
mod utils;

use debug::DebugPlugin;
use game::GamePlugin;
use physics::PhysicsPlugin;
use camera::CameraPlugin;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(CameraPlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}
