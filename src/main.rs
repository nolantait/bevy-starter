use bevy::prelude::*;

use starter::{camera::CameraPlugin, debug::DebugPlugin, game::GamePlugin, physics::PhysicsPlugin};

const BACKGROUND_COLOR: Color = Color::srgb(0.4, 0.4, 0.4);

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy game".into(),
                resizable: false,
                resolution: (800., 600.).into(),
                canvas: Some("#bevy".to_owned()),
                desired_maximum_frame_latency: core::num::NonZero::new(1u32),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((DebugPlugin, GamePlugin, CameraPlugin, PhysicsPlugin))
        .run();
}
