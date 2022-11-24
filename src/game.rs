use bevy::{
    app::App,
    prelude::{Plugin, Color, ClearColor},
};

use crate::physics::PhysicsPlugin;
use crate::camera::CameraPlugin;
use crate::boids::BoidPlugin;
use crate::input::InputPlugin;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(BACKGROUND_COLOR))
            .add_plugin(PhysicsPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(InputPlugin)
            .add_plugin(BoidPlugin);
    }
}
