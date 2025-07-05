#![allow(unused)]
use bevy::prelude::*;

#[derive(Resource)]
pub struct MousePosition(Vec2);

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(MousePosition(Vec2::default()));
}
