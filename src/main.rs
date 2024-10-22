use bevy::prelude::*;

mod camera;
mod debug;
mod game;
mod input;
mod physics;
mod utils;

use camera::CameraPlugin;
use debug::DebugPlugin;
use game::GamePlugin;
// use physics::PhysicsPlugin;

const BACKGROUND_COLOR: Color = Color::srgb(0.4, 0.4, 0.4);

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy game".to_string(), // ToDo
                resolution: (800., 600.).into(),
                canvas: Some("#bevy".to_owned()),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((DebugPlugin, GamePlugin, CameraPlugin))
        .run();
}
