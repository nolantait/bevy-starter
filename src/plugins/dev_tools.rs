//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

mod pausing;

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, |mut options: ResMut<UiDebugOptions>| {
        options.toggle().run_if(input_just_pressed(TOGGLE_KEY))
    })
    .add_plugins(pausing::PausePlugin);
}
