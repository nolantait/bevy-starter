use bevy::prelude::*;

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

const BACKGROUND_COLOR: Color = Color::rgb(0.4, 0.4, 0.4);

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
        .add_plugins((
            DebugPlugin,
            GamePlugin,
            CameraPlugin,
            PhysicsPlugin
        ))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
