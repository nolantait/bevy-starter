//! Pause, resume and step through the game.

use std::time::Duration;

use bevy::prelude::*;

/// Plugin for pausing and stepping through the game.
pub(super) struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PauseState>()
            .add_systems(
                OnEnter(PauseState::Paused),
                |mut time: ResMut<Time<Physics>>| time.pause(),
            )
            .add_systems(
                OnExit(PauseState::Paused),
                |mut time: ResMut<Time<Physics>>| time.unpause(),
            )
            .add_systems(Update, pause_button)
            .add_systems(Update, step_button.run_if(in_state(PauseState::Paused)));
    }
}

/// Controls whether or not the game should be paused.
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum PauseState {
    /// The game is paused.
    Paused,
    /// The game is running.
    #[default]
    Running,
}

/// Toggles the pause state when the pause button is pressed.
fn pause_button(
    current_state: ResMut<State<PauseState>>,
    mut next_state: ResMut<NextState<PauseState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyP) {
        let new_state = match current_state.get() {
            PauseState::Paused => PauseState::Running,
            PauseState::Running => PauseState::Paused,
        };
        next_state.set(new_state);
    }
}

/// Advances the game by a small amount of delta time when the step button is pressed.
fn step_button(mut time: ResMut<Time<Physics>>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Enter) {
        time.advance_by(Duration::from_secs_f64(1.0 / 60.0));
    }
}
