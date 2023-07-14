use bevy::app::App;
use bevy::prelude::Plugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                LogDiagnosticsPlugin::default(),
                FrameTimeDiagnosticsPlugin::default()
            ));
    }
}
